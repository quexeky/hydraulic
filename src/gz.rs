use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

pub struct Gzip {

}
impl Algorithm for Gzip {
    fn finalise_encode(&self, data: &[u8], level: usize) -> Result<&[u8], CompressionError> {
        todo!()
    }

    fn finalise_decode(&self, data: &[u8]) -> Result<&[u8], DecompressionError> {
        todo!()
    }

    fn partial_encode(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        todo!()
    }

    fn partial_decode(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }
}