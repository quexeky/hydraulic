use std::io;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

pub trait Algorithm {
    fn encode(&self, data: &[u8], level: usize) -> Result<Vec<u8>, CompressionError>;
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError>;
    fn partial_encode(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
    fn partial_decode(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError>;
}