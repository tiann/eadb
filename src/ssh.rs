use crate::{
    exec::{check_call, check_output, run_pty},
    remote_op::RemoteOp,
};
use anyhow::{Result, ensure};

#[derive(Debug, Clone, PartialEq)]
pub struct Ssh {
    uri: String,
    pass: Option<String>,
    port: Option<u32>,
}

impl Ssh {
    pub fn new(uri: impl Into<String>, pass: Option<String>, port: Option<u32>) -> Self {
        Ssh {
            uri: uri.into(),
            pass,
            port
        }
    }

    fn get_cmd_prefix(&self, cmd: &str) -> String {
        let scp = cmd == "scp";
        if let Some(pass) = &self.pass {
            format!("sshpass -p {} {} {}", pass, cmd, self.get_port_str(scp))
        } else {
            format!("{} {}", cmd, self.get_port_str(scp))
        }
    }

    fn get_port_str(&self, scp: bool) -> String {
        if let Some(port) = &self.port {
            if scp {
                // scp use -P port(Upercase P) while ssh use -p port(lowercase p)
                format!("-P {}", port)
            } else {
                format!("-p {}", port)
            }
        } else {
            "".to_string()
        }
    }
}

impl RemoteOp for Ssh {
    fn check_connection(&self) -> Result<()> {
        let code = run_pty(format!(
            "{} {} -o ConnectTimeout=3 -o ConnectionAttempts=1 -q exit",
            self.get_cmd_prefix("ssh"),
            self.uri
        ))?;

        ensure!(code == 0, "failed to connect to SSH server");

        Ok(())
    }

    fn shell(&self, cmd: &str) -> Result<()> {
        // [sshpass -p self.pass] ssh self.uri cmd
        run_pty(format!(
            "{} {} {}",
            self.get_cmd_prefix("ssh"),
            self.uri,
            cmd
        ))
        .map(|_| ())
    }

    fn check_call(&self, cmd: &str) -> Result<()> {
        check_call(format!(
            "{} {} {}",
            self.get_cmd_prefix("ssh"),
            self.uri,
            cmd
        ))
    }

    fn check_output(&self, cmd: &str) -> Result<String> {
        check_output(cmd)
    }

    fn push(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} -r {} {}:{}",
            self.get_cmd_prefix("scp"),
            src,
            self.uri,
            dst
        ))
    }

    fn pull(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} -r {}:{} {}",
            self.get_cmd_prefix("scp"),
            self.uri,
            src,
            dst
        ))
    }
}
