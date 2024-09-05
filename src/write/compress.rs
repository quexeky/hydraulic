use std::io;
use std::io::Write;

use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::CompressionLevel;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WriteEncoder<'a, T: Algorithm, D: Write> {
    meta: AlgorithmMeta,
    algorithm: &'a T,
    buffer: Vec<u8>,
    pub destination: D,
}

impl<'a, T: Algorithm, D: Write> WriteEncoder<'a, T, D> {
    pub fn new(algorithm: &'a T, destination: D, level: CompressionLevel) -> Self {
        Self {
            meta: AlgorithmMeta { level: Some(level) },
            algorithm,
            destination,
            buffer: Vec::new(),
        }
    }

    /// Sends data to a buffer
    pub fn queue(&mut self, data: &[u8]) {
        self.buffer.extend(data);
    }

    /// Compresses all data within the buffer and writes it to disk
    // TODO: Add in errors
    pub fn write(&mut self) -> io::Result<()> {
        let data = self
            .algorithm
            .partial_encode(&self.buffer, &self.meta)
            .unwrap();
        self.destination.write_all(&data)?;
        self.buffer.clear();
        Ok(())
    }

    /// Finalises the data that is being written to disk and writes any tail data. Returns to File
    /// object where the compressed data is now stored
    pub fn finish(mut self) -> Result<D, CompressionError> {
        let enc = self.algorithm.finalise_encode(&self.meta)?;
        self.queue(&enc);
        self.write().unwrap();
        Ok(self.destination)
    }
}
