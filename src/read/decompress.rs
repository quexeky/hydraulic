use std::fs::File;
use std::io::Read;

use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::decompression_error::DecompressionError;

pub struct ReadDecoder<'a, T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: &'a T,
    origin: File
}

impl<'a, T: Algorithm> ReadDecoder<'a, T> {
    pub fn new(alg: &'a T) -> Self {
        return Self {
            meta: AlgorithmMeta { level: None },
            encoder: alg,
            origin: File::open("hydraulic").unwrap(),
        }
    }

    /// Attempts to write a buffer and returns how many bytes were written to the writer.
    // TODO: Add in errors
    pub fn read<const S: usize>(&mut self) -> Result<Vec<u8>, DecompressionError> {
        let mut buf: [u8; S] = [0; S];
        self.origin.read_exact(&mut buf).unwrap();
        return self.encoder.partial_decode(&buf, &self.meta);
    }

    /// Attempts to read and decompress all remaining data within the origin file
    pub fn read_all(&mut self) -> Result<Vec<u8>, DecompressionError> {
        let mut buf = Vec::new();
        self.origin.read_to_end(&mut buf).unwrap();

        return self.encoder.partial_decode(&*buf, &self.meta);
    }

    /// Finalises the data that is being written to disk and writes any tail data. Returns to File
    /// object where the compressed data is now stored
    pub fn finish(mut self) -> Result<Vec<u8>, DecompressionError> {
        Ok(Vec::from(self.encoder.finalise_decode(&self.meta).unwrap()))
    }
}