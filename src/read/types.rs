use std::io::Read;

// Algorithms which enable the "read" feature must implement this trait.
//
pub trait ReadAlgorithm {
    fn init<T: Into<Vec<u8>>>(data: T) -> dyn Read;
    fn read_sized(&self, buf: &[u8]) -> Result<(), ()>; // TODO: Add actual result return values
    fn read_sized_in_place<const S: usize>(&self) -> [u8; S];
    fn read_all(self) -> Vec<u8>;
}

