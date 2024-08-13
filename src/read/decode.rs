use std::io::{BufReader, Read};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;

pub struct ReadDecoder<'a, T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: &'a [u8]
}

impl<'a, T: Algorithm> ReadDecoder<'a, T> {
    pub fn new(alg: &'a T, data: &'a [u8]) -> Self {
        let decoded = ;

        return Self {
            meta: AlgorithmMeta { level: None },
            encoder: alg,
            data: &[0],
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

    /// Completes the buffer, decodes the data, and then returns the decompressed data as a result
    pub fn finish(self) -> io::Result<Vec<u8>> {
        let self_data = self.data;
        let data = self_data.get_ref();

        let self_encoder = self.encoder;

        let res = self_encoder.decode(data).unwrap();
        Ok(res)
    }
}