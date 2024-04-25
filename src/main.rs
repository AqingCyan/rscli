use std::fs;
use std::path::Path;

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Parser)]
#[command(name = "rscli", version, author, about, long_about)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let player: Player = result?;
                ret.push(player);
            }

            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }

    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
