mod parser;

use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
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

#[derive(Default)]
struct Dashboard {
    values: BTreeMap<String, String>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }
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
    let mut dashboard = Dashboard::new();
    let mut reader = BufReader::new(stderr).lines();
    while let Some(line) = reader.next_line().await? {
        if let Ok(pairs) = parser::parse(&line) {
            for (k, v) in pairs {
                dashboard.set(k, v);
            }
        }
    }
    Ok(())
}
