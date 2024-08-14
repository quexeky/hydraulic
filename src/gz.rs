use crate::AlgorithmMeta;
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

pub struct Gzip {}
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
