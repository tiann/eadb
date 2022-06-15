use anyhow::Result;

pub trait RemoteOp {
    /// check if remote connection is available
    fn check_connection(&self) -> Result<()>;

    /// run remote shell and enter pty
    fn shell(&self, cmd: &str) -> Result<()>;

    /// call remote command and stream stdout
    fn check_call(&self, cmd: &str) -> Result<()>;

    /// call remote command and get output
    fn check_output(&self, cmd: &str) -> Result<String>;

    /// push src file to dst
    fn push(&self, src: &str, dst: &str) -> Result<()>;

    /// pull src file to dst
    fn pull(&self, src: &str, dst: &str) -> Result<()>;
}