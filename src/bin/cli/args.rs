use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug)]
// #[command(version, about, long_about)]
struct Args {
    /// Compression algorithm
    algorithm: Algorithm,
    
    /// Data to 
    
}

#[derive(Debug)]
enum Algorithm {
    Gz,
}