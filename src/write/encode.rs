use std::io;
use std::io::{BufWriter, Write};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;

pub struct WriteEncoder<'a, T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: &'a T,
    data: BufWriter<Vec<u8>>
}

impl<'a, T: Algorithm> WriteEncoder<'a, T> {
    pub fn new(alg: &'a T) -> Self {
        return Self {
            meta: AlgorithmMeta {},
            encoder: alg,
            data: BufWriter::new(Vec::new()),
        }
    }

    /// Attempts to write a buffer and returns how many bytes were written to the writer
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.data.write(data)
    }

    /// Attempts to write everything within a buffer to this writer
    pub fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        self.data.write_all(data)
    }

    /// Flushes the output stream and ensures that all bytes reach their destination
    pub fn flush(&mut self) -> io::Result<()> {
        self.data.flush()
    }

    /// Completes the buffer, encodes the data, and then returns the decompressed data as a result
    pub fn finish(self) -> io::Result<Vec<u8>> {
        let self_data = self.data;
        let data = self_data.buffer();

        let self_encoder = self.encoder;

        let res = self_encoder.encode(data, 10).unwrap();
        Ok(res)
    }
}