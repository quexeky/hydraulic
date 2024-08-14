use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;
use crate::AlgorithmMeta;
use std::io;

pub trait Algorithm {
    fn finalise_encode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError>;
    fn finalise_decode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError>;
    fn partial_encode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError>;
    fn partial_decode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError>;
}
