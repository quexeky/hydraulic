use hydraulic::implementation::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Gzip {}
impl Gzip {
    pub fn new() -> Self {
        Self {}
    }
}
impl Algorithm for Gzip {
    fn finalise_encode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        // Final encoding steps go here
        todo!()
    }

    fn finalise_decode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        // Final decoding steps go here
        todo!()
    }

    fn partial_encode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError> {
        // Implement your decompression of a single data block here
        todo!()
    }

    fn partial_decode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError> {
        // Implement your compression of a single data block here
        todo!()
    }
}

fn main() {

}