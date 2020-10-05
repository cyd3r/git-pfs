#[macro_use]
extern crate anyhow;
use clap::Clap;

mod paths;
mod add_file;
mod storage_locate;
mod synchronize;
mod unlink_file;

/// Poor man's Git LFS
#[derive(Clap)]
#[clap(version = "0.1", author = "cyd3r <cyd3rhacker@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Add a file to the storage or update an existing one
    Add(AddFileArgs),
    /// Remove a file link to the storage but keep them on the fileystem
    #[clap(name = "unlink")]
    UnlinkFile(UnlinkFileArgs),
    /// Synchronize with the storage
    #[clap(name = "sync")]
    Synchronize,
    /// Return the storage path for a local file
    Locate(LocateArgs),
}

#[derive(Clap)]
struct AddFileArgs {
    /// The file that should be added
    filename: String
}
#[derive(Clap)]
struct UnlinkFileArgs {
    /// The file that should be unlinked from the storage
    filename: String
}
#[derive(Clap)]
struct LocateArgs {
    /// Local filename
    filename: String
}

fn parse_commands() -> anyhow::Result<()> {
    match Opts::parse().subcmd {
        SubCommand::Add(args) => {
            add_file::add_file(args.filename)?;
        },
        SubCommand::UnlinkFile(args) => {
            unlink_file::unlink_file(args.filename)?;
        },
        SubCommand::Synchronize => {
            synchronize::synchronize()?;
        },
        SubCommand::Locate(args) => {
            storage_locate::locate(args.filename)?;
        },
    }
    Ok(())
}

fn main() {
    if let Err(err) = parse_commands() {
        println!("{}", err);
        std::process::exit(1);
    }
}
