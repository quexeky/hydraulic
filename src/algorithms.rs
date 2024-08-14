use std::io;
use crate::AlgorithmMeta;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

pub trait Algorithm {
    fn finalise_encode(&self, meta: &AlgorithmMeta) -> Result<&[u8], CompressionError>;
    fn finalise_decode(&self, meta: &AlgorithmMeta) -> Result<&[u8], DecompressionError>;
    fn partial_encode(&self, data: &[u8], meta: &AlgorithmMeta) -> Result<&[u8], CompressionError>;
    fn partial_decode(&self, data: &[u8], meta: &AlgorithmMeta) -> Result<&[u8], DecompressionError>;
}