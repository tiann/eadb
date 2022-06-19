use crate::{
    constants::to_rootfs_dir,
    exec::{check_call, check_output, run_pty},
    remote_op::RemoteOp,
};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Adb {
    serial: Option<String>,
}

impl Adb {
    pub fn new(serial: Option<String>) -> Self {
        Adb { serial }
    }

    fn get_cmd_prefix(&self) -> String {
        if let Some(serial) = &self.serial {
            format!("adb -s {}", serial)
        } else {
            "adb".to_string()
        }
    }
}

impl RemoteOp for Adb {
    fn check_connection(&self) -> Result<()> {
        let code = run_pty(format!("{} get-state", self.get_cmd_prefix()))?;
        if code != 0 {
            return Err(anyhow::anyhow!("failed to connect to device"));
        }

        // todo: check multiple device connection

        let id = check_output(format!("{} shell id -u", self.get_cmd_prefix()))?;
        if id.trim() != "0" {
            return Err(anyhow::anyhow!("adb root is necessary!"));
        }
        Ok(())
    }

    fn shell(&self, cmd: &str) -> Result<()> {
        run_pty(format!("{} shell {}", self.get_cmd_prefix(), cmd)).map(|_| ())
    }

    fn check_call(&self, cmd: &str) -> Result<()> {
        check_call(format!("{} shell {}", self.get_cmd_prefix(), cmd))
    }

    fn push(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} push {} {}",
            self.get_cmd_prefix(),
            src,
            to_rootfs_dir(dst)
        ))
    }

    fn pull(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} pull {} {}",
            self.get_cmd_prefix(),
            to_rootfs_dir(src),
            dst
        ))
    }

    fn check_output(&self, cmd: &str) -> Result<String> {
        check_output(cmd)
    }
}
