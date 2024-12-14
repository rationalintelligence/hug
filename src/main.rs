mod dashboard;
mod parser;

use crate::dashboard::Dashboard;
use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::process::Stdio;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::{RwLock, RwLockReadGuard},
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

#[derive(Default, Clone)]
pub struct State {
    values: Arc<RwLock<BTreeMap<String, String>>>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.write().await.insert(key.into(), value.into());
    }

    pub async fn read(&self) -> RwLockReadGuard<BTreeMap<String, String>> {
        self.values.read().await
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<BTreeMap<String, String>> {
        self.values.blocking_read()
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

    let stderr = child.stdout.take().ok_or(CliError::Stderr)?;
    let mut state = State::new();

    let mut dashboard = Dashboard::start(state.clone());

    let mut reader = BufReader::new(stderr).lines();
    while let Some(line) = reader.next_line().await? {
        if let Ok(pairs) = parser::parse(&line) {
            for pair in pairs {
                state.set(pair.key, pair.value).await;
            }
        }
    }

    dashboard.stop();
    dashboard.join();

    Ok(())
}
