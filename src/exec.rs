use subprocess::{Exec, ExitStatus};
use anyhow::Result;

pub fn check_output(cmd: impl Into<String>) -> Result<String> {
    let command = cmd.into();
    println!("check_output: {}", &command);
    let cap = subprocess::Exec::shell(command).capture()?;
    Ok(cap.stdout_str())
}

pub fn run_pty(cmd: impl Into<String>) -> Result<u32> {
    let command = cmd.into();
    println!("run pty: {command}",);
    if let ExitStatus::Exited(code) = Exec::shell(command).join()? {
        Ok(code)
    } else {
        Err(anyhow::anyhow!("failed to run pty"))
    }
}

pub fn check_call(cmd: impl Into<String>) -> Result<()> {
    let command = cmd.into();
    println!("check_call: {}", &command);
    Exec::shell(command).join()?;
    Ok(())
}

// call a command ignore its output
pub fn call(cmd: impl Into<String>) {
    let _ = check_call(cmd);
}