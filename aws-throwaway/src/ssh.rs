use anyhow::{anyhow, Result};
use async_trait::async_trait;
use russh::{
    client::{Config, Handle, Handler},
    ChannelMsg, Sig,
};
use russh_keys::{key::PublicKey, PublicKeyBase64};
use std::{fmt::Display, io::Write, net::IpAddr, path::Path, sync::Arc, time::Duration};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
};

pub struct SshConnection {
    address: IpAddr,
    session: Handle<Client>,
}

impl SshConnection {
    pub async fn new(
        stream: TcpStream,
        address: IpAddr,
        host_public_key_bytes: Vec<u8>,
        client_private_key: &str,
    ) -> Result<Self> {
        let config = Config {
            // By default ssh has no keep alive, resulting in idle connections dying.
            // So set a timeout of 60 seconds to prevent losing idle connections in this way.
            keepalive_interval: Some(Duration::from_secs(60)),
            ..Default::default()
        };

        let key = Arc::new(
            russh_keys::decode_secret_key(client_private_key, None)
                .map_err(|e| anyhow!(e).context("Failed to connect to ssh server"))?,
        );
        let mut session = russh::client::connect_stream(
            Arc::new(config),
            stream,
            Client {
                host_public_key_bytes,
            },
        )
        .await?;
        if session.authenticate_publickey("ubuntu", key).await.unwrap() {
            tracing::info!("Succesfully connected to {address} over ssh");
            Ok(SshConnection { session, address })
        } else {
            Err(anyhow!("Authentication with ssh server failed"))
        }
    }

    /// Runs a shell command, returning its stdout + stderr when the command finishes.
    ///
    /// An informative panic will occur if
    /// * the process returns a non-zero exit code
    /// * the process is killed by a signal other than TERM or KILL
    ///
    /// This method is recommended for running setup commands that must succeed.
    pub async fn shell(&self, command: &str) -> CommandOutput {
        tracing::info!("running command on {}: {}", self.address, command);

        let mut channel = self.session.channel_open_session().await.unwrap();
        channel.exec(true, command).await.unwrap();
        let mut stdout = vec![];
        let mut stderr = vec![];
        let mut status = None;
        let mut failed = None;
        while let Some(msg) = channel.wait().await {
            match msg {
                ChannelMsg::Data { data } => stdout.write_all(&data).unwrap(),
                ChannelMsg::ExtendedData { data, ext } => {
                    if ext == 1 {
                        stderr.write_all(&data).unwrap()
                    } else {
                        tracing::warn!("received unknown extended data with extension type {ext} containing: {:?}", data.to_vec())
                    }
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    status = Some(exit_status);
                    // cant exit immediately, there might be more data still
                }
                ChannelMsg::ExitSignal {
                    signal_name,
                    core_dumped,
                    error_message,
                    ..
                } => {
                    failed = Some(format!(
                    "killed via signal {signal_name:?} core_dumped={core_dumped} {error_message:?}"
                ))
                }
                _ => {}
            }
        }
        let output = CommandOutput {
            stdout: String::from_utf8(stdout).unwrap(),
            stderr: String::from_utf8(stderr).unwrap(),
        };

        check_results(&format!("The command {command}"), failed, status, &output);
        output
    }

    /// Runs a shell command reporting its logs over stdout while it executes.
    ///
    /// Every line received is sent to the receiver as a String.
    ///
    /// An error will be sent to the receiver if:
    /// * the process returns a non-zero exit code
    /// * the process is killed by a signal other than TERM or KILL
    ///
    /// This method is recommended for running a long running service or application under test.
    pub async fn shell_stdout_lines(
        &self,
        command: &str,
    ) -> tokio::sync::mpsc::UnboundedReceiver<Result<String>> {
        let task = format!(
            "running shell_stdout_lines on {}: {}",
            self.address, command
        );
        tracing::info!("{task}");

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut channel = self.session.channel_open_session().await.unwrap();
        channel.exec(true, command).await.unwrap();
        let command = command.to_owned();
        tokio::task::spawn(async move {
            let mut stdout = vec![];
            let mut stderr = vec![];
            loop {
                tokio::select! {
                    msg = channel.wait() => {
                        match msg {
                            Some(msg) => {
                                match msg {
                                    ChannelMsg::Data { data } => {
                                        stdout.write_all(&data).unwrap();
                                        while let Some((until, _)) = stdout.iter().enumerate().find(|(_, c)| **c == b'\n') {
                                            let old = stdout.split_off(until + 1);
                                            stdout.pop(); // remove the '\n'
                                            if tx.send(Ok(String::from_utf8(stdout).unwrap())).is_err() {
                                                return;
                                            }
                                            stdout = old;
                                        }
                                    }
                                    ChannelMsg::ExtendedData { data, ext } => {
                                        if ext == 1 {
                                            stderr.write_all(&data).unwrap()
                                        } else {
                                            tracing::warn!("received unknown extended data with extension type {ext} containing: {:?}", data.to_vec())
                                        }
                                    }
                                    ChannelMsg::ExitStatus { exit_status } => {
                                        if exit_status != 0 {
                                            let stderr = String::from_utf8(stderr.clone()).unwrap();
                                            let err = anyhow!("command {command}\nUnexpectedly exited with status {exit_status:?}\nstderr:{stderr}");
                                            if tx.send(Err(err)).is_err() {
                                                return;
                                            }
                                        }
                                    }
                                    ChannelMsg::ExitSignal {
                                        signal_name,
                                        core_dumped,
                                        error_message,
                                        ..
                                    } => {
                                        if !matches!(signal_name, Sig::TERM | Sig::KILL) {
                                            let stderr = String::from_utf8(stderr.clone()).unwrap();
                                            let err = anyhow!("command {command}\nWas unexpectedly killed via signal {signal_name:?} core_dumped={core_dumped}\n{error_message:?}\nstderr:{stderr}");
                                            if tx.send(Err(err)).is_err() {
                                                return;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            None => return,
                        }
                    },
                    _ = tx.closed() => {
                        return;
                    },
                };
            }
        });

        rx
    }

    /// Push a file from the local machine to the remote machine
    pub async fn push_file(&self, source: &Path, dest: &Path) {
        let task = format!("pushing file from {source:?} to {}:{dest:?}", self.address);
        tracing::info!("{task}");

        let source = File::open(source)
            .await
            .map_err(|e| anyhow!(e).context(format!("Failed to read from {source:?}")))
            .unwrap();
        self.push_file_impl(&task, source, dest).await;
    }

    /// Create a file on the remote machine at `dest` with the provided bytes.
    pub async fn push_file_from_bytes(&self, bytes: &[u8], dest: &Path) {
        let task = format!("pushing raw bytes to {}:{dest:?}", self.address);
        tracing::info!("{task}");

        let source = BufReader::new(bytes);
        self.push_file_impl(&task, source, dest).await;
    }

    async fn push_file_impl<R: AsyncReadExt + Unpin>(&self, task: &str, source: R, dest: &Path) {
        let mut channel = self.session.channel_open_session().await.unwrap();
        let command = format!("cat > '{0}'\nchmod 777 {0}", dest.to_str().unwrap());
        channel.exec(true, command).await.unwrap();

        let mut stdout = vec![];
        let mut stderr = vec![];
        let mut status = None;
        let mut failed = None;
        channel.data(source).await.unwrap();
        channel.eof().await.unwrap();
        while let Some(msg) = channel.wait().await {
            match msg {
                ChannelMsg::Data { data } => stdout.write_all(&data).unwrap(),
                ChannelMsg::ExtendedData { data, ext } => {
                    if ext == 1 {
                        stderr.write_all(&data).unwrap()
                    } else {
                        tracing::warn!("received unknown extended data with extension type {ext} containing: {:?}", data.to_vec())
                    }
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    status = Some(exit_status);
                    // cant exit immediately, there might be more data still
                }
                ChannelMsg::ExitSignal {
                    signal_name,
                    core_dumped,
                    error_message,
                    ..
                } => {
                    failed = Some(format!(
                    "killed via signal {signal_name:?} core_dumped={core_dumped} {error_message:?}"
                ))
                }
                _ => {}
            }
        }
        let output = CommandOutput {
            stdout: String::from_utf8(stdout).unwrap(),
            stderr: String::from_utf8(stderr).unwrap(),
        };

        check_results(task, failed, status, &output);
    }

    /// Pull a file from the remote machine to the local machine
    pub async fn pull_file(&self, source: &Path, dest: &Path) {
        let task = format!("pulling file from {}:{source:?} to {dest:?}", self.address);
        tracing::info!("{task}");

        let mut channel = self.session.channel_open_session().await.unwrap();
        let command = format!("cat '{}'", source.to_str().unwrap());
        channel.exec(true, command).await.unwrap();

        let mut out = File::create(dest).await.unwrap();
        let mut stderr = vec![];
        let mut status = None;
        let mut failed = None;
        channel.eof().await.unwrap();
        while let Some(msg) = channel.wait().await {
            match msg {
                ChannelMsg::Data { data } => tokio::io::AsyncWriteExt::write_all(&mut out, &data)
                    .await
                    .unwrap(),
                ChannelMsg::ExtendedData { data, ext } => {
                    if ext == 1 {
                        stderr.write_all(&data).unwrap()
                    } else {
                        tracing::warn!("received unknown extended data with extension type {ext} containing: {:?}", data.to_vec())
                    }
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    status = Some(exit_status);
                    // cant exit immediately, there might be more data still
                }
                ChannelMsg::ExitSignal {
                    signal_name,
                    core_dumped,
                    error_message,
                    ..
                } => {
                    failed = Some(format!(
                    "killed via signal {signal_name:?} core_dumped={core_dumped} {error_message:?}"
                ))
                }
                _ => {}
            }
        }

        let output = String::from_utf8(stderr).unwrap();
        check_results(&task, failed, status, &output);
    }
}

fn check_results<T: Display>(
    task: &str,
    failed: Option<String>,
    exit_status: Option<u32>,
    output: &T,
) {
    if let Some(failed) = failed {
        panic!("{task:?} was {failed}\n{output}")
    }

    match exit_status {
        Some(status) => {
            if status != 0 {
                panic!("{task} failed with exit code {status}\n{output}")
            }
        }
        None => panic!("{task} did not exit cleanly\n{output}"),
    }
}

#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

impl Display for CommandOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.stdout.is_empty() {
            write!(f, "stdout:\n{}", self.stdout)?;
        }
        if !self.stderr.is_empty() {
            write!(f, "stderr:\n{}", self.stderr)?;
        }
        Ok(())
    }
}

struct Client {
    host_public_key_bytes: Vec<u8>,
}

#[async_trait]
impl Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(&mut self, host_public_key: &PublicKey) -> Result<bool, Self::Error> {
        let result = host_public_key.public_key_bytes() == self.host_public_key_bytes;
        if !result {
            // This is just a debug because the actual error is bubbled up via russh
            tracing::debug!(
                "ssh keys mismatched\n{:?}\n{:?}",
                host_public_key.public_key_bytes(),
                self.host_public_key_bytes
            );
        }
        Ok(result)
    }
}
