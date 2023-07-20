use std::{net::IpAddr, time::Duration};

use tokio::{net::TcpStream, time::Instant};

use crate::ssh::SshConnection;

pub struct Ec2Instance {
    public_ip: IpAddr,
    private_ip: IpAddr,
    client_private_key: String,
    host_public_key_bytes: Vec<u8>,
    host_public_key: String,
    ssh: SshConnection,
}

impl Ec2Instance {
    /// Use this address to connect to this instance from outside of AWS
    pub fn public_ip(&self) -> IpAddr {
        self.public_ip
    }

    /// Use this address to connect to this instance from within AWS
    pub fn private_ip(&self) -> IpAddr {
        self.private_ip
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
        format!("{} {}", &self.public_ip, &self.host_public_key)
    }

    /// Returns an object that allows commands to be sent over ssh
    pub fn ssh(&self) -> &SshConnection {
        &self.ssh
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
            self.public_ip()
        )
    }

    pub(crate) async fn new(
        public_ip: IpAddr,
        private_ip: IpAddr,
        host_public_key_bytes: Vec<u8>,
        host_public_key: String,
        client_private_key: &str,
    ) -> Self {
        loop {
            let start = Instant::now();
            // We retry many times before we are able to succesfully make an ssh connection.
            // Each error is expected and so is logged as a `info!` that describes the underlying startup process that is supposed to cause the error.
            // A numbered comment is left before each `info!` to demonstrate the order each error occurs in.
            match tokio::time::timeout(Duration::from_secs(10), TcpStream::connect((public_ip, 22)))
                .await
            {
                Err(_) => {
                    // 1.
                    tracing::info!("Timed out connecting to {public_ip} over ssh, the host is probably not accessible yet, retrying");
                    continue;
                }
                Ok(Err(e)) => {
                    // 2.
                    tracing::info!("failed to connect to {public_ip}:22, the host probably hasnt started their ssh service yet, retrying, error was {e}");
                    tokio::time::sleep_until(start + Duration::from_secs(1)).await;
                    continue;
                }
                Ok(Ok(stream)) => {
                    match SshConnection::new(
                        stream,
                        public_ip,
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
                                ssh,
                                public_ip,
                                private_ip,
                                host_public_key_bytes,
                                host_public_key,
                                client_private_key: client_private_key.to_owned(),
                            };
                        }
                    };
                }
            };
        }
    }
}
