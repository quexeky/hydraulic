use std::io;

pub trait Algorithm {
    fn encode(self, data: &[u8]) -> io::Result<&[u8]>;
    fn decode(self, data: &[u8]) -> io::Result<&[u8]>;
}