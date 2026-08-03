use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

mod util;

#[derive(Debug, Parser)]
#[command(version, about = "Lazy8, A CHIP-8 Emulator written in Rust")]
pub struct RunArgs {
    /// Specify whether the program should output debug logs
    #[arg(short, long)]
    pub debug_mode: bool,
    /// Specify a rom file to load
    #[arg(short, long)]
    pub rom_file: Option<String>,
}

pub fn parse_args() -> RunArgs {
    RunArgs::parse()
}

pub fn init_logger(use_debug_mode: bool) -> Result<(), fern::InitError> {
    let level = if util::is_release_build() {
        if use_debug_mode {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    } else {
        LevelFilter::Trace
    };
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Blue)
        .trace(Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] ({}) [{}]: {}",
                chrono::Local::now().format("%I:%M:%S %p"),
                match record.target() {
                    "lazy8_ui" => "UI",
                    "lazy8_core" => "CORE",
                    _ => record.target(),
                },
                colors.color(record.level()),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
