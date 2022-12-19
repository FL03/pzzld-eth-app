/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root, BoxResult, Bundle};
use clap::{Args, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize, ValueEnum)]
pub enum Mode {
    #[default]
    Dev = 0,
    Prod = 1
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Auto(Auto),
    Compile(Compile),
    Setup(Setup),
    Start {
        #[clap(value_enum)]
        mode: Option<Mode>,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Auto(auto) => {
                tracing::info!("Initializing the CI/CD pipeline...");
            }
            Self::Compile(compile) => {
                tracing::info!("Compiling the codebase...");
                compile.handler()?;
            }
            Self::Setup(setup) => {
                tracing::info!("Setting up the environment...");
                setup.handler()?;
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

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Auto {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    release: bool
}

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Compile {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    release: bool
}

impl Compile {
    pub fn new(release: bool) -> Self {
        Self { release }
    }
    fn artifacts(&self) -> BoxResult<&Self> { 
        if std::fs::create_dir_all(&dist_dir()).is_err() {
            tracing::info!("Clearing out the previous build");
            std::fs::remove_dir_all(&dist_dir())?;
            std::fs::create_dir_all(&dist_dir())?;
        }
        Ok(self)
    }
    fn commands(&self) -> BoxResult<&Self> {
        npm(vec!["run", "build"])?;
        copy_dir_all(
            &project_root().join("frontend/build"), 
            project_root().join(".artifacts/dist/build")
        )?;
        copy_dir_all(
            &project_root().join("wasm/pkg"), 
            project_root().join(".artifacts/dist/wasm")
        )?;
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        self.artifacts()?;
        self.commands()?;
        Ok(self)
    }
}


#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Setup {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool
}

impl Setup {
    pub fn new(extras: bool) -> Self {
        Self { extras }
    }
    fn commands(&self) -> BoxResult<&Self> {
        command("rustup", vec!["default", "nightly"])?;
        command("rustup", vec!["target", "add", "wasm32-unknown-unknown", "wasm32-wasi", "--toolchain", "nightly"])?;
        if self.extras.clone() {
            command("rustup", vec!["component", "add", "clippy", "rustfmt", "--toolchain", "nightly"])?;
        }
        command("npm", vec!["install", "-g", "wasm-pack"])?;
        command("npm", vec!["install"])?;
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        self.commands()?;
        Ok(self)
    }
}
