use crate::{Algorithm, AlgorithmMeta};
use crate::implementation::{CompressionError, DecompressionError};

pub struct NoCompression();

impl NoCompression {
    pub fn new() -> Self {
        Self {}
    }
}
impl Algorithm for NoCompression {
    fn finalise_encode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        Ok(vec![])
    }

    fn finalise_decode(&self, _meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        Ok(vec![])
    }

    fn partial_encode(&self, data: &[u8], _meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        Ok(data.to_vec())
    }

    fn partial_decode(&self, data: &[u8], _meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        Ok(data.to_vec())
    }
}