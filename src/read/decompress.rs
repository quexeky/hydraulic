use std::io::Read;

use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::decompression_error::DecompressionError;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReadDecoder<'a, T: Algorithm, D: Read> {
    meta: AlgorithmMeta,
    algorithm: &'a T,
    pub origin: D,
}

impl<'a, T: Algorithm, D: Read> ReadDecoder<'a, T, D> {
    pub fn new(algorithm: &'a T, origin: D) -> Self {
        Self {
            meta: AlgorithmMeta { level: None },
            algorithm,
            origin,
        }
    }

    /// Attempts to read data of size equal to the buffer. Returns a Vec<u8> of the decompressed
    /// resulting data
    // TODO: Add in errors
    pub fn read(&mut self, buf: &mut [u8]) -> Result<Vec<u8>, DecompressionError> {
        self.origin.read_exact(buf).unwrap();
        self.algorithm.partial_decode(buf, &self.meta)
    }

    /// Attempts to read and decompress all data within the origin until EOF
    pub fn read_all(&mut self) -> Result<Vec<u8>, DecompressionError> {
        let mut buf = Vec::new();
        self.origin.read_to_end(&mut buf).unwrap();

        self.algorithm.partial_decode(&buf, &self.meta)
    }

    /// Attempts to finalise the data which remains within the buffer. This function makes
    /// no assumption on how much data remains within the origin
    pub fn finish(self) -> Result<Vec<u8>, DecompressionError> {
        let data = self.algorithm.finalise_decode(&self.meta);
        println!("Data: {:?}", data?);
        self.algorithm.finalise_decode(&self.meta)
    }
}
