#[cfg(test)]
mod tests;

mod algorithm_meta;
mod algorithms;
mod compression_level;
mod deflate;
mod errors;
mod features;
pub mod read;
mod write;

/// Generic interface to compress data through either a Read or a Write interface
/// TODO: Example
pub mod compress {
    pub use crate::read::compress::ReadEncoder;
    pub use crate::write::compress::WriteEncoder;
}
/// Generic interface to decompress data through either a Read or a Write interface
pub mod decompress {
    pub use crate::read::decompress::ReadDecoder;
    pub use crate::write::decompress::WriteDecoder;
}
/// Implements:
///  - ReadDecoder
///  - WriteDecoder
///  - ReadEncoder
///  - WriteEncoder
///  - Algorithm
///  - AlgorithmMeta
///  - CompressionLevel
pub mod prelude {
    pub use crate::decompress::ReadDecoder;
    pub use crate::decompress::WriteDecoder;

    pub use crate::compress::ReadEncoder;
    pub use crate::compress::WriteEncoder;

    pub use crate::AlgorithmMeta;
    pub use crate::CompressionLevel;
}
/// Metadata for algorithms to add generic data to. Should not be used to store anything except
/// data specifically for the algorithm itself
pub use crate::algorithm_meta::AlgorithmMeta;
/// Algorithm is the generic type which implements all the functions required for both encoding
/// and decoding
pub use crate::algorithms::Algorithm;
/// General Compression Level implementation for Algorithm.
/// Low = 2
/// Med = 6
/// High = 9
/// Custom values may also be assigned using the CompressionLevel::Custom(x) enum type
pub use crate::compression_level::CompressionLevel;

#[cfg(feature = "gzip")]
pub use features::gz::Gzip;

#[cfg(feature = "gzip")]
pub use crate::deflate::Deflate;
