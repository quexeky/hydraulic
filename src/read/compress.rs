use std::io::{BufReader};
use crate::{AlgorithmMeta, CompressionLevel};
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;
use crate::read::decompress::ReadDecoder;

pub struct ReadEncoder<'a, T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: &T,
    data: &'a [u8]
}

impl<'a, T: Algorithm> ReadEncoder<'a, T> {
    pub fn new(alg: &'a T, data: &'a [u8], compression_level: CompressionLevel) -> Self {

        return Self {
            meta: AlgorithmMeta { level: Some(compression_level) },
            encoder: alg,
            data,
        }
    }

    /// Attempts to read as many bytes as is specified by the size
    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, CompressionError> {
        let mut new;
        (new, self.data) = self.data.split_at(size);
        self.encoder.partial_encode(new)
    }

    /// Attempts to read everything that remains within the Reader
    pub fn read_all(&mut self) -> Result<Vec<u8>, CompressionError> {
        self.encoder.partial_encode(self.data)
    }
}