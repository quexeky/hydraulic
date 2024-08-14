use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::decompression_error::DecompressionError;
use crate::write::decompress::WriteDecoder;

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
    pub fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.origin.read(buffer).unwrap();
        self.encoder.partial_decode();
        return Ok(0);
    }

    /// Attempts to write all provided data to the buffer
    pub fn read_all(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buffer.extend(data);
        return Ok(0);
    }

    /// Compresses all data within the buffer and writes it to disk
    // TODO: Add in errors
    pub fn flush(&mut self) -> io::Result<()> {
        let data = self.encoder.partial_decode(self.buffer, &self.meta).unwrap();
        self.destination.write_all(data).unwrap();
        self.buffer.clear();
        return Ok(())
    }

    /// Finalises the data that is being written to disk and writes any tail data. Returns to File
    /// object where the compressed data is now stored
    pub fn finish(mut self) -> io::Result<File> {
        let enc = self.encoder.finalise_decode(&self.meta).unwrap();
        self.write_all(enc).unwrap();
        self.flush().unwrap();
        Ok(self.destination)
    }
}