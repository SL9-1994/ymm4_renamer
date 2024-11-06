use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum RenameOption {
    KitsuneYukkuri,
    // 未対応
}

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the folder where each part of the illustration is stored.
    #[arg(short, long)]
    pub input_folder: PathBuf,

    /// Please select the format of your illustration.
    #[arg(short, long, value_enum, default_value_t=RenameOption::KitsuneYukkuri)]
    pub rename_option: RenameOption,

    /// Sets the logging level
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

impl Default for Args {
    fn default() -> Self {
        Args::parse()
    }
}

impl Args {
    pub fn new() -> Self {
        Self::default()
    }
}
