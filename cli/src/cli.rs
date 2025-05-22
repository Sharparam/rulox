use clap::{Parser, ValueEnum};
use tracing::Level;

// pub const EX_DATAERR: u8 = 65;
// pub const EX_SOFTWARE: u8 = 70;
// pub const EX_IOERR: u8 = 74;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to a Lox program to execute
    pub path: Option<String>,

    /// Code to run, overrides files and stdin
    #[clap(short = 'e')]
    pub code: Option<String>,

    /// Enter the REPL after executing the program, or immediately if no program given
    #[clap(short, long)]
    pub repl: bool,

    /// Enable verbose output, equivalent to setting the trace log level
    #[clap(short, long)]
    pub verbose: bool,

    /// Log level to use
    #[clap(short, long, value_enum, default_value_t = LogLevel::Warn)]
    level: LogLevel,

    /// Disassemble the input program
    #[clap(short, long)]
    pub disassemble: bool,
}

impl Args {
    pub fn log_level(&self) -> Level {
        match self.level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}
