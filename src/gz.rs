use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;
pub struct Gzip {

}
impl Algorithm for Gzip {
    fn encode(&self, data: &[u8], level: usize) -> Result<Vec<u8>, CompressionError> {
        let data = compress_to_vec(data, level as u8);
        return Ok(data);
    }

    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
        let data = decompress_to_vec(data);
        return match data {
            Ok(o) => Ok(o),
            Err(e) => { println!("{}", e); Err(DecompressionError::Generic)}
        }
    }
}