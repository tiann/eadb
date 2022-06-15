use crate::{remote_op::RemoteOp, exec::{run_pty, check_output, check_call}};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Ssh {
    uri: String,
    pass: Option<String>,
}

impl Ssh {
    pub fn new(uri: impl Into<String>, pass: Option<String>) -> Self {
        Ssh {
            uri: uri.into(),
            pass,
        }
    }

    fn get_cmd_prefix(&self, cmd: &str) -> String {
        if let Some(pass) = &self.pass {
            format!("sshpass -p {} {}", pass, cmd)
        } else {
            cmd.to_string()
        }
    }
}

impl RemoteOp for Ssh {
    fn check_connection(&self) -> Result<()> {
        let code = run_pty(format!(
            "{} {} -o ConnectTimeout=3 -o ConnectionAttempts=1 -q exit",
            self.get_cmd_prefix("ssh"), self.uri
        ))?;
        if code == 0 {
            Ok(())
        } else {
            Err(anyhow::anyhow!("failed to connect to SSH server"))
        }
    }

    fn shell(&self, cmd: &str) -> Result<()> {
        // [sshpass -p self.pass] ssh self.uri cmd
        run_pty(format!("{} {} {}", self.get_cmd_prefix("ssh"), self.uri, cmd)).map(|_| ())
    }

    fn check_call(&self, cmd: &str) -> Result<()> {
        check_call(format!("{} {} {}", self.get_cmd_prefix("ssh"), self.uri, cmd))
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