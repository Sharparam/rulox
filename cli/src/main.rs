mod cli;
mod logging;

use std::{
    fs,
    io::{self, Read, Write},
};

use crate::cli::Args;
use anyhow::{Context, Result};
use clap::Parser;
use rulox::{
    compiler,
    vm::{self, InterpretResult},
};
use tracing::Level;

fn main() -> Result<()> {
    let args = Args::parse();
    let log_level = if args.verbose {
        Level::TRACE
    } else {
        args.log_level()
    };
    logging::init_logging(log_level);

    let mut out = &mut io::stdout();
    let mut err = &mut io::stderr();
    let mut vm = vm::VM::new(&mut out, &mut err);

    if args.repl {
        return repl();
    }

    let contents = get_program_contents(&args).context("Failed to get program contents")?;

    interpret(&contents).context("Failed to interpret source")
}

fn repl() -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_ok() {
            interpret(&input)?;
        } else {
            println!();
        }
    }
}

fn interpret(source: &str) -> InterpretResult {
    compiler::compile(source);

    Ok(())
}

fn get_program_contents(args: &Args) -> Result<String> {
    if let Some(code) = &args.code {
        return Ok(code.to_string());
    }

    match &args.path {
        Some(path) => read_program_from_file(path),
        None => read_program_from_stdin(),
    }
}

fn read_program_from_file(path: &str) -> Result<String> {
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read program from {}", &path))?;

    Ok(contents)
}

fn read_program_from_stdin() -> Result<String> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;
    Ok(contents)
}
