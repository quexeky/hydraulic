use std::io;
use std::io::{BufReader, Read};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::decompression_error::DecompressionError;

pub struct ReadDecoder<'a, T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: &'a T,
    data: &'a [u8]
}

impl<'a, T: Algorithm> ReadDecoder<'a, T> {
    pub fn new(alg: &'a T, data: &'a [u8]) -> Self {

        return Self {
            meta: AlgorithmMeta { level: None },
            encoder: alg,
            data,
        }
    }

    /// Attempts to read as many bytes as is specified by the size
    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, DecompressionError> {
        let mut new;
        (new, self.data) = self.data.split_at(size);
        self.encoder.partial_decode(new)
    }

    /// Attempts to read everything that remains within the Reader
    pub fn read_all(&mut self) -> Result<Vec<u8>, DecompressionError> {
        self.encoder.partial_decode(self.data)
    }
}