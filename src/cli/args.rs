use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_stdin::{FileOrStdin, MaybeStdin};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about)]
pub struct Cli {
    #[arg(value_enum)]
    ///// Compression algorithm
    pub (crate) algorithm: Algorithm,
    
    /// Data to pass into the compressor
    pub (crate) data: FileOrStdin,

    #[arg(value_enum)]
    ///// Read or Write
    pub (crate) read_write: ReadWrite,

    /// Size (in bytes) at which data is sent through the compressor
    pub (crate) rate: u64,

}
#[derive(ValueEnum, Debug, Clone)]
pub enum ReadWrite {
    Read,
    Write
}

#[derive(ValueEnum, Debug, Clone)]
enum Algorithm {
    #[cfg(any(feature = "gzip", feature = "full"))]
    Gz,
    #[cfg(any(feature = "deflate", feature = "full"))]
    Deflate,
}