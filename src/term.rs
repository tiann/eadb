use std::io::Write;
use anyhow::Result;
use termcolor::{StandardStream, ColorChoice, ColorSpec, WriteColor};

pub fn print_color(msg: impl Into<String>, color: termcolor::Color) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    Ok(writeln!(&mut stdout, "{}", msg.into())?)
}

pub fn print_err(msg: impl Into<String>) {
    let m = msg.into();
    if print_color(&m, termcolor::Color::Red).is_err() {
        println!("{}", &m);
    }
}

pub fn print_tip(msg: impl Into<String>) {
    let m = msg.into();
    if print_color(&m, termcolor::Color::Green).is_err() {
        println!("{}", &m);
    }
}