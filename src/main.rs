mod misc;
mod run;

use crate::misc::*;
use std::{path::PathBuf, sync::Arc};

#[derive(clap::Parser)]
pub struct Cli {
    /// database path
    db: PathBuf,

    /// path to hook (gets invoked for each file with the file as first argument)
    hook: PathBuf,

    /// optional path to logfile
    logfile: Option<PathBuf>,

    /// set maximum processed filesize
    max_filesize: Option<String>,

    /// enable multiprocessing
    use_mp: bool,

    /// suppress log messages
    suppress_logmsgs: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Run { listing_file: PathBuf },

    RunGlob { base: PathBuf, globs: Vec<String> },
}

fn main() {
    let cli = <Cli as clap::Parser>::parse();

    {
        use simplelog::*;
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }

    let sigdat = Arc::new(SignalDataIntern::new());
    register_signal_handlers(sigdat.clone());

    let dbt = sled::Config::new()
        .path(&cli.db)
        .mode(sled::Mode::HighThroughput)
        .cache_capacity(1_048_576)
        .use_compression(true)
        .compression_factor(5)
        .open()
        .expect("unable to open database");

    // disable catching of Ctrl-C
    sigdat.set_ctrlc_armed(true);

    use crate::run::IngestList;
    let ingl = match &cli.command {
        Command::Run { listing_file } => IngestList::IndexFile(&listing_file),
        Command::RunGlob { base, globs } => IngestList::GlobPattern(&base, &globs[..]),
    };

    handle_dbres(crate::run::run(&cli, &dbt, &sigdat, ingl));
    handle_dbres(dbt.flush());
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert()
    }
}
