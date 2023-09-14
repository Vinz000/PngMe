use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::types;

pub fn encode(args: EncodeArgs) -> types::Result<()> {
    let input_bytes = fs::read(&args.input)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    let output = args.output.unwrap_or(args.input);
    let mut png = Png::try_from(input_bytes.as_slice())?;
    png.append_chunk(chunk);
    fs::write(output, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: DecodeArgs) -> types::Result<()> {
    let png_bytes = fs::read(args.input)?;
    let png = Png::try_from(png_bytes.as_slice())?;
    let chunk = png.chunk_by_type(&args.chunk_type);
    match chunk {
        Some(c) => println!("{}", c),
        None => println!("No message found"),
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> types::Result<()> {
    let png_bytes = fs::read(&args.input)?;
    let mut png = Png::try_from(png_bytes.as_slice())?;
    let removed_chunk = png.remove_chunk(&args.chunk_type)?;
    fs::write(&args.input, png.as_bytes())?;
    println!("Removed: {:?}", removed_chunk.to_string());

    Ok(())
}

pub fn print(args: PrintArgs) -> types::Result<()> {
    let png_bytes = fs::read(&args.input)?;
    let png = Png::try_from(png_bytes.as_slice())?;
    println!("{}", png);
    Ok(())
}
