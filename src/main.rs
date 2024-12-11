use std::fs;

use alloy_primitives::hex;
use clap::Parser;
use op_alloy_protocol::{compress_brotli, BrotliLevel};

#[derive(Parser, Debug)]
#[command(author, version, about = "Compresses data using Brotli compression")]
struct Cli {
    /// Brotli compression level (9, 10, or 11)
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(9..=11))]
    level: u32,

    /// Hex-encoded input data
    #[arg(short, long, required_unless_present = "file")]
    data: Option<String>,

    /// Input file containing hex-encoded data
    #[arg(short = 'f', long, conflicts_with = "data")]
    file: Option<String>,
}

fn main() -> eyre::Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();

    // Convert the compression level to BrotliLevel
    let brotli_level = match args.level {
        9 => BrotliLevel::Brotli9,
        10 => BrotliLevel::Brotli10,
        11 => BrotliLevel::Brotli11,
        _ => unreachable!("Clap ensures level is within range"),
    };

    let data = if let Some(file_path) = args.file {
        let file_content = fs::read_to_string(file_path)?;
        hex::decode(file_content.trim())?
    } else if let Some(data_str) = args.data {
        hex::decode(data_str)?
    } else {
        unreachable!("Clap ensures that either data or file is provided.")
    };

    dbg!(data.len());

    // Compress the data
    let compressed_data = compress_brotli(&data, brotli_level)?;

    dbg!(compressed_data.len());

    Ok(())
}
