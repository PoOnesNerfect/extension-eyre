use extension_eyre::{eyre::Report, eyre::WrapErr, Extension, ExtensionExt};
use snafu::Snafu;
use std::process::Command;
use tracing::instrument;

pub struct Retry(bool);

#[instrument]
fn main() -> Result<(), Report> {
    install_tracing();
    extension_eyre::install()?;

    if let Err(err) = read_config() {
        if let Some(Retry(true)) = err.extension_ref() {
            println!("retrying read config\n");
            read_config()?;
        }
    }

    Ok(())
}

#[instrument]
fn read_config() -> Result<String, Report> {
    read_file("fake_file").wrap_err("Unable to read config")
}

#[instrument]
fn read_file(path: &str) -> Result<String, Report> {
    Command::new("cat").arg(path).output2()
}

trait Output {
    fn output2(&mut self) -> Result<String, Report>;
}

impl Output for Command {
    #[instrument]
    fn output2(&mut self) -> Result<String, Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            OutputSnafu { i: 0usize }.fail().extension(Retry(true))
        } else {
            Ok(stdout.into())
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("cmd exited with non-zero status code {i}"))]
pub struct OutputError {
    i: usize,
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
