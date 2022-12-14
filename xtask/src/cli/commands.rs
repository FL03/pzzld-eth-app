/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root, BoxResult, Bundle};
use clap::{Subcommand, ValueEnum};
use std::process::Command;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Mode {
    #[default]
    Dev = 0,
    Prod = 1
}

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Setup {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        extras: bool,
    },
    Start {
        #[clap(value_enum)]
        mode: Option<Mode>,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
                if std::fs::create_dir_all(&dist_dir()).is_err() {
                    tracing::info!("Clearing out the previous build");
                    std::fs::remove_dir_all(&dist_dir())?;
                    std::fs::create_dir_all(&dist_dir())?;
                }
                npm(vec!["run", "build"])?;
                copy_dir_all(
                    &project_root().join("build"), 
                    project_root().join(".artifacts/dist/build")
                )?;
                copy_dir_all(
                    &project_root().join("wasm/pkg"), 
                    project_root().join(".artifacts/dist/wasm")
                )?;
            }
            Self::Setup { extras } => {
                tracing::info!("Setting up the environment...");
                command("rustup", vec!["default", "nightly"])?;
                command("rustup", vec!["target", "add", "wasm32-unknown-unknown", "wasm32-wasi", "--toolchain", "nightly"])?;
                if *extras {
                    command("rustup", vec!["component", "add", "clippy", "rustfmt", "--toolchain", "nightly"])?;
                }
                command("npm", vec!["install", "-g", "wasm-pack"])?;
                command("npm", vec!["install"])?;
            }
            Self::Start { mode } => {
                tracing::info!("Initializing the application server...");
                match mode.unwrap_or_default().clone() as i32 {
                    0 => {
                        npm(vec!["run", "dev"])?;
                    },
                    _ => { npm(vec!["run", "start"])?; }
                }
                
            }
        };
        Ok(self)
    }
}
///
pub fn command(program: &str, args: Vec<&str>) -> BoxResult {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
    Ok(())
}
///
pub fn npm(args: Vec<&str>) -> BoxResult {
    command("npm", args)
}
