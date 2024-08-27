use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

/// UNIMPLEMENTED!
/// This module provides implementations for the Gzip compression algorithm
pub struct Gzip {}
impl Gzip {
    pub fn new() -> Self {
        Self {}
    }
}
impl Algorithm for Gzip {
    fn finalise_encode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        todo!()
    }

    fn finalise_decode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }

    fn partial_encode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError> {
        todo!()
    }

    fn partial_decode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }
}
