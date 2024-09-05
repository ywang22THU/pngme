use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about=None)]
#[command(about = "Operate your secret info with png files!")]
pub struct Cli{
    #[command(subcommand)]
    pub command: PngMeArgs
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    /// Encode something in your file!
    Encode(EncodeArgs),
    /// Decode something in the special chunk and print it out!
    Decode(DecodeArgs),
    /// Remove something in the special chunk!
    Remove(RemoveArgs),
    /// Print all hidden info out!
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    /// The input file path
    pub file_path: PathBuf,
    /// The chunk type you want to decode
    pub chunk_type: String,
    /// Secret!
    pub message: String,
    /// The option output file, default to be your input
    pub output_file: Option<PathBuf>
}

#[derive(Args)]
pub struct DecodeArgs {
    /// The input file path
    pub file_path: PathBuf,
    /// The chunk type you want to decode
    pub chunk_type: String
}

#[derive(Args)]
pub struct RemoveArgs {
    /// The input file path
    pub file_path: PathBuf,
    /// The chunk type you want to decode
    pub chunk_type: String
}

#[derive(Args)]
pub struct PrintArgs {
    /// The input file path
    pub file_path: PathBuf
}
