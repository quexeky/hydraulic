use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

use miniz_oxide::{deflate, inflate};

/// UNIMPLEMENTED!
/// This module provides implementations for the Gzip compression algorithm
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Gzip {}
impl Gzip {}
impl Algorithm for Gzip {
    fn finalise_encode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        Ok(vec![])
    }

    fn finalise_decode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        Ok(vec![])
    }

    fn partial_encode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError> {
        let res = deflate::compress_to_vec(data, meta.level.unwrap().to_integer() as u8);
        Ok(res)
    }

    fn partial_decode(
        &self,
        data: &[u8],
        _meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError> {
        let res = inflate::decompress_to_vec(data);
        println!("Partial Decode res: {:?}", res);
        Ok(res.unwrap())
    }
}
