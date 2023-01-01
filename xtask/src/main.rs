/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod args;
pub mod commands;
pub mod workspace;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    CommandLineInterface::new().handler()?;

    Ok(())
}

// pub fn handle_build_files() {
//     let yaml = load_yaml!("cli.yml");
//     let matches = CommandFactory::from_yaml(yaml).get_matches();
//     let verbosity =
//     matches.value_of("VERBOSE")
//     .and_then(|s| s.parse::<u8>().ok())
//     .unwrap();
//     println!("{}", verbosity);
// }


#[derive(Clone, Debug, Hash, Parser, PartialEq)]
#[clap(about, author, version)]
#[clap(long_about = None)]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<commands::Commands>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub debug: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn handler(&self) -> Result<&Self> {
        if self.debug {}
        else if let Some(cmds) = &self.command {
            cmds.handler()?;
        }
        Ok(self)
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}