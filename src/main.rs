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
    #[arg(short, long, required_unless_present = "file")]
    data: Option<String>,
    #[arg(short = 'f', long, conflicts_with = "data")]
    file: Option<String>,
}

fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    let brotli_level = match args.level {
        9 => BrotliLevel::Brotli9,
        10 => BrotliLevel::Brotli10,
        11 => BrotliLevel::Brotli11,
        _ => unreachable!(),
    };

    // let data = if let Some(file_path) = args.file {
    //     let file_content = fs::read_to_string(file_path)?;
    //     hex::decode(file_content.trim())?
    // } else if let Some(data_str) = args.data {
    //     hex::decode(data_str)?
    // } else {
    //     unreachable!();
    // };

    // println!("");
    // println!("Original size: {:?}", data.len());
    // let compressed_data = compress_brotli(&data, brotli_level)?;
    // println!("Compressed size: {:?}", compressed_data.len());

    let mut full_proofs = vec![];
    let mut compressed_padded_proofs = vec![];
    let mut compressed_proofs = vec![];

    let num_proofs = 100_000;
    for _ in 0..num_proofs {
        let full_proof = (0..256).map(|_| rand::random::<u8>()).collect::<Vec<u8>>();

        full_proofs.extend(&full_proof);

        compressed_padded_proofs.extend(&full_proof[..128].to_vec());
        compressed_padded_proofs.extend(vec![0u8; 128]);

        compressed_proofs.extend(full_proof[..128].to_vec());
    }

    println!("Num proofs: {}", num_proofs);
    println!("\nUncompressed full proofs size: {:?}", full_proofs.len());
    let compressed_full_8 = compress_brotli(&full_proofs, brotli_level)?;
    println!("Compressed full proofs size: {:?}", compressed_full_8.len());

    println!(
        "\nUncompressed 'compressed proofs' with padding size: {:?}",
        compressed_padded_proofs.len()
    );
    let compressed_padded_4 = compress_brotli(&compressed_padded_proofs, brotli_level)?;
    println!(
        "Compressed 'compressed proofs' with padding size: {:?}",
        compressed_padded_4.len()
    );

    println!(
        "\nUncompressed 'compressed proof' size: {:?}",
        compressed_proofs.len()
    );
    let compressed_no_padding_4 = compress_brotli(&compressed_proofs, brotli_level)?;
    println!(
        "Compressed 'compressed proof' size: {:?}",
        compressed_no_padding_4.len()
    );

    Ok(())
}
