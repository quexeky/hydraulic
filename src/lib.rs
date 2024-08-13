pub mod write;
pub mod read;
pub mod algorithms;
pub mod gz;
pub mod errors;
#[cfg(test)]
pub mod tests;

use std::io::BufRead;

pub struct AlgorithmMeta {
    level: Option<CompressionLevel>

}
pub enum CompressionLevel {
    High,
    Med,
    Low,
    Custom(usize)
}
impl CompressionLevel {
    pub fn to_integer(&self) -> usize {
        match self {
            CompressionLevel::High => { 10 }
            CompressionLevel::Med => { 6 }
            CompressionLevel::Low => { 2 }
            CompressionLevel::Custom(x) => { *x }
        }
    }
}