use clap::{Parser, Subcommand};
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    /// Compression algorithm
    algorithm: Algorithm,
    
    /// Data to pass into the compressor
    data: FileOrStdin,
}

#[derive(Subcommand, Debug, Clone)]
#[command(version, about, long_about)]
enum Algorithm {
    #[cfg(any(feature = "gzip", feature = "full"))]
    Gz,
    #[cfg(any(feature = "deflate", feature = "full"))]
    Deflate,
}