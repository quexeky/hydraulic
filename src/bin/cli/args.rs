use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug)]
// #[command(version, about, long_about)]
struct Args {
    /// Compression algorithm
    algorithm: Algorithm,
    
    /// Data to pass into the compressor
    data: FileOrStdin,
}

#[derive(Debug)]
enum Algorithm {
    #[cfg(any(feature = "gzip", feature = "full"))]
    Gz,
    #[cfg(any(feature = "deflate", feature = "full"))]
    Deflate,
}