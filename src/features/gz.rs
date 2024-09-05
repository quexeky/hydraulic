use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

/// UNIMPLEMENTED!
/// This module provides implementations for the Gzip compression algorithm
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Gzip {}
impl Gzip {}
impl Algorithm for Gzip {
    fn finalise_encode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        todo!()
    }

    fn finalise_decode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }

    fn partial_encode(
        &self,
        _data: &[u8],
        _meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError> {
        todo!()
    }

    fn partial_decode(
        &self,
        _data: &[u8],
        _meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }
}
