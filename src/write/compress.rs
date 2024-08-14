use std::fs::File;
use std::io;
use std::io::Write;

use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::{AlgorithmMeta, CompressionLevel};

pub struct WriteEncoder<'a, T: Algorithm, D: Write> {
    meta: AlgorithmMeta,
    algorithm: &'a T,
    buffer: Vec<u8>,
    destination: D,
}

impl<'a, T: Algorithm, D: Write> WriteEncoder<'a, T, D> {
    pub fn new(algorithm: &'a T, destination: D, level: CompressionLevel) -> Self {
        return Self {
            meta: AlgorithmMeta { level: Some(level) },
            algorithm,
            destination,
            buffer: Vec::new(),
        };
    }

    /// Attempts to write a buffer and returns how many bytes were written to the writer.
    // TODO: Add in errors
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buffer.extend(data);
        return Ok(0);
    }

    /// Attempts to write all provided data to the buffer
    pub fn write_all(&mut self, data: &[u8]) -> Result<(), CompressionError> {
        self.buffer.extend(data);
        return Ok(());
    }

    /// Compresses all data within the buffer and writes it to disk
    // TODO: Add in errors
    pub fn flush(&mut self) -> io::Result<()> {
        let data = self
            .algorithm
            .partial_encode(&*self.buffer, &self.meta)
            .unwrap();
        self.destination.write_all(&*data).unwrap();
        self.buffer.clear();
        return Ok(());
    }

    /// Finalises the data that is being written to disk and writes any tail data. Returns to File
    /// object where the compressed data is now stored
    pub fn finish(mut self) -> Result<D, CompressionError> {
        let enc = self.algorithm.finalise_encode(&self.meta).unwrap();
        self.write_all(&*enc).unwrap();
        self.flush().unwrap();
        Ok(self.destination)
    }
}
