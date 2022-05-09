mod cli;
mod logging;

use std::{
    fs,
    io::{self, Read},
};

use crate::cli::Args;
use anyhow::{Context, Result};
use clap::Parser;
use tracing::{info, Level};

fn main() -> Result<()> {
    let args = Args::parse();
    let log_level = if args.verbose {
        Level::TRACE
    } else {
        args.log_level()
    };
    logging::init_logging(log_level);

    let contents = get_program_contents(&args).context("Failed to get program contents")?;

    info!("Program loaded! Contents:");
    println!("{}", contents);

    Ok(())
}

fn get_program_contents(args: &Args) -> Result<String> {
    if let Some(code) = &args.code {
        return Ok(code.to_string());
    }

    match &args.path {
        Some(path) => read_program_from_file(&path),
        None => read_program_from_stdin(),
    }
}

fn read_program_from_file(path: &str) -> Result<String> {
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read program from {}", &path))?;
    return Ok(contents);
}

fn read_program_from_stdin() -> Result<String> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;
    return Ok(contents);
}
