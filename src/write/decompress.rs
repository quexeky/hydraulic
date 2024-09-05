use std::io;
use std::io::Write;

use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::decompression_error::DecompressionError;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WriteDecoder<'a, T: Algorithm, D: Write> {
    meta: AlgorithmMeta,
    algorithm: &'a T,
    buffer: Vec<u8>,
    pub destination: D,
}

impl<'a, T: Algorithm, D: Write> WriteDecoder<'a, T, D> {
    pub fn new(algorithm: &'a T, destination: D) -> Self {
        Self {
            meta: AlgorithmMeta { level: None },
            algorithm,
            destination,
            buffer: Vec::new(),
        }
    }

    /// Attempts to write a buffer and returns how many bytes were written to the writer.
    pub fn queue(&mut self, data: &[u8]) {
        self.buffer.extend(data);
    }


    /// Compresses all data within the buffer and writes it to disk
    pub fn write(&mut self) -> io::Result<()> {
        let data = self
            .algorithm
            .partial_decode(&self.buffer, &self.meta)
            .unwrap();
        self.destination.write_all(&data)?;
        self.buffer.clear();
        Ok(())
    }

    /// Finalises the data that is being written to disk and writes any tail data. Returns to File
    /// object where the compressed data is now stored
    pub fn finish(mut self) -> Result<D, DecompressionError> {
        let enc = self.algorithm.finalise_decode(&self.meta)?;
        self.queue(&enc);
        self.write().unwrap();
        Ok(self.destination)
    }
}
