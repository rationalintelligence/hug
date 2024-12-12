use anyhow::Result;
use clap::Parser;
use std::process::Stdio;
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[derive(Error, Debug)]
enum CliError {
    #[error("Can't get access to stderr")]
    Stderr,
    #[error("Command is not set")]
    Command,
}

#[derive(Parser, Debug)]
struct Opts {
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut args = opts.command.into_iter();
    let command = args.next().ok_or(CliError::Command)?;

    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stderr = child.stderr.take().ok_or(CliError::Stderr)?;

    let mut reader = BufReader::new(stderr).lines();
    while let Some(line) = reader.next_line().await? {
        println!("{line}");
    }
    Ok(())
}
