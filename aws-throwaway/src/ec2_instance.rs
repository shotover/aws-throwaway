use crate::ssh::SshConnection;
use crate::Aws;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::{net::TcpStream, time::Instant};

/// Represents a currently running EC2 instance and provides various methods for interacting with the instance.
///
/// This type implements serde Serialize/Deserialize to allow you to save and restore the instance from disk.
/// After restoring Ec2Instance in this way you need to call the [`Ec2Instance::init`] method.
#[derive(Serialize, Deserialize)]
pub struct Ec2Instance {
    pub(crate) aws_id: String,
    connect_ip: IpAddr,
    public_ip: Option<IpAddr>,
    private_ip: IpAddr,
    client_private_key: String,
    host_public_key_bytes: Vec<u8>,
    host_public_key: String,
    #[serde(skip)]
    ssh: Option<SshConnection>,
    network_interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkInterface {
    pub private_ipv4: Ipv4Addr,
    pub device_index: i32,
}

impl Ec2Instance {
    /// Use this address to connect to this instance from outside of AWS
    pub fn public_ip(&self) -> Option<IpAddr> {
        self.public_ip
    }

    /// Use this address to connect to this instance from within AWS
    pub fn private_ip(&self) -> IpAddr {
        self.private_ip
    }

    /// Use this address to get the private or public IP that aws-throwaway is using to ssh to the instance.
    /// Whether or not this is public is decided by [`crate::AwsBuilder::use_public_addresses`].
    ///
    /// You should use this address if you want to connect to the instance from your local machine
    pub fn connect_ip(&self) -> IpAddr {
        self.connect_ip
    }

    /// List of all network interfaces attached to this instance.
    /// Includes the primary interface that has the ip returned by [`Ec2Instance::private_ip`] as well as all other interfaces attached to this instance at the time it was created.
    pub fn network_interfaces(&self) -> &[NetworkInterface] {
        &self.network_interfaces
    }

    /// Use this as the private key of your machine when connecting to this instance
    pub fn client_private_key(&self) -> &str {
        &self.client_private_key
    }

    /// Use this for authenticating a host programmatically
    pub fn host_public_key_bytes(&self) -> &[u8] {
        &self.host_public_key_bytes
    }

    /// Insert this into your known_hosts file to avoid errors due to unknown fingerprints
    pub fn openssh_known_hosts_line(&self) -> String {
        format!("{} {}", &self.connect_ip, &self.host_public_key)
    }

    /// Returns an object that allows commands to be sent over ssh
    pub fn ssh(&self) -> &SshConnection {
        self.ssh
            .as_ref()
            .expect("Make sure to call `Ec2Instance::init` after deserializing `Ec2Instance`")
    }

    /// Get a list of commands that the user can paste into bash to manually open an ssh connection to this instance.
    pub fn ssh_instructions(&self) -> String {
        format!(
            r#"```
chmod 700 key 2> /dev/null || true
echo '{}' > key
echo '{}' > known_hosts
chmod 400 key
TERM=xterm ssh -i key ubuntu@{} -o "UserKnownHostsFile known_hosts"
```"#,
            self.client_private_key(),
            self.openssh_known_hosts_line(),
            self.connect_ip
        )
    }

    /// Delete this instance.
    /// Prefer using [`Aws::cleanup_resources`] at end of runtime as it will automatically destroy all resources, not just this one instance.
    /// However this method can be useful when you have a single instance that you would like to terminate before the rest.
    ///
    /// `Aws` instance must be passed in manually here since `Ec2Instance`s can be deserialized when there is no `Aws` instance.
    pub async fn terminate(self, aws: &Aws) {
        aws.terminate_instance(self).await;
    }

    /// It is gauranteed that public_ip will be Some if use_public_address is true
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn new(
        aws_id: String,
        connect_ip: IpAddr,
        public_ip: Option<IpAddr>,
        private_ip: IpAddr,
        host_public_key_bytes: Vec<u8>,
        host_public_key: String,
        client_private_key: &str,
        network_interfaces: Vec<NetworkInterface>,
    ) -> Self {
        loop {
            let start = Instant::now();
            // We retry many times before we are able to succesfully make an ssh connection.
            // Each error is expected and so is logged as a `info!` that describes the underlying startup process that is supposed to cause the error.
            // A numbered comment is left before each `info!` to demonstrate the order each error occurs in.
            match tokio::time::timeout(
                Duration::from_secs(10),
                TcpStream::connect((connect_ip, 22)),
            )
            .await
            {
                Err(_) => {
                    // 1.
                    tracing::info!("Timed out connecting to {connect_ip} over ssh, the host is probably not accessible yet, retrying");
                    continue;
                }
                Ok(Err(e)) => {
                    // 2.
                    tracing::info!("failed to connect to {connect_ip}:22, the host probably hasnt started their ssh service yet, retrying, error was {e}");
                    tokio::time::sleep_until(start + Duration::from_secs(1)).await;
                    continue;
                }
                Ok(Ok(stream)) => {
                    match SshConnection::new(
                        stream,
                        connect_ip,
                        host_public_key_bytes.clone(),
                        client_private_key,
                    )
                    .await
                    {
                        Err(err) => {
                            // 3.
                            tracing::info!("Failed to make ssh connection to server, the host has probably not run its user-data script yet, retrying, error was: {err:?}");
                            tokio::time::sleep_until(start + Duration::from_secs(1)).await;
                            continue;
                        }
                        // 4. Then finally we have a working ssh connection.
                        Ok(ssh) => {
                            break Ec2Instance {
                                aws_id,
                                connect_ip,
                                ssh: Some(ssh),
                                public_ip,
                                private_ip,
                                host_public_key_bytes,
                                host_public_key,
                                client_private_key: client_private_key.to_owned(),
                                network_interfaces,
                            };
                        }
                    };
                }
            };
        }
    }

    /// After deserializing [`Ec2Instance`] this method must be called to recreate the ssh connection.
    ///
    /// No need to call after creating via [`crate::Aws::create_ec2_instance`]
    pub async fn init(&mut self) -> Result<()> {
        let connect_ip = self.connect_ip;

        // We use a drastically simplifed initialization approach here compared to `Ec2Instance::new`.
        // Since we can assume that the server has either already started up or is now terminated we
        // avoid retries and tailor our error messages in order to provide better error reporting.
        let stream =
            tokio::time::timeout(Duration::from_secs(5), TcpStream::connect((connect_ip, 22)))
                .await
                .map_err(|_| anyhow!("Timed out connecting to {connect_ip}:22"))?
                .with_context(|| format!("Failed to connect to {connect_ip}:22"))?;

        let ssh = SshConnection::new(
            stream,
            connect_ip,
            self.host_public_key_bytes.clone(),
            &self.client_private_key,
        )
        .await
        .with_context(|| format!("Failed to make ssh connection to {connect_ip}:22"))?;
        self.ssh = Some(ssh);

        Ok(())
    }
}
