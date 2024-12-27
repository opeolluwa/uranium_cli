use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub new: Commands,
}

///
/// the cli action can be one of `app`, or `gateway`
/// ## Example
/// ```sh
/// ucli new app --path=.
/// ucli new gateway --path=.
/// ```
#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Commands {
    New ,
}
