use anyhow::anyhow;
use anyhow::Result;
use clap::{Parser, Subcommand};
use nedry::lang::lang_list;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,

    #[arg(short, long)]
    /// The entry point file.
    /// 
    /// Will analyzing dependency in this file.
    input: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Bundle all dependency to single file 
    Bundle {
        #[arg(short, long)]
        /// Output file path
        output: Option<PathBuf>,
    },

    /// Build file to an executable file
    /// 
    /// This will bundle file and invoke compiler
    Build {
        #[arg(short, long)]
        /// Output file path
        output: Option<PathBuf>,
    },

    /// Build & Run file
    /// 
    /// This will invoke executable file that generate by `build` command
    Run,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let list = lang_list();

    let lang = list
        .get(
            cli.input
                .extension()
                .ok_or_else(|| anyhow!("File extension not found"))?
                .to_str()
                .ok_or_else(|| anyhow!("Path contain non utf8 character"))?,
        )
        .ok_or_else(|| anyhow!("Unknown language"))?;

    match cli.cmd {
        Commands::Bundle { output } => {
            let mut output =
                BufWriter::new(fs::File::create(output.unwrap_or_else(|| "out.c".into()))?);
            lang.bundle(cli.input.canonicalize()?, &mut output as &mut dyn Write)?
        }
        Commands::Build { output } => lang.build(cli.input, output)?,
        Commands::Run => lang.run(cli.input)?,
    }
    Ok(())
}
