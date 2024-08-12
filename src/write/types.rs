pub trait WriteAlgorithm<'a> {
    fn init() -> Box<Self>;
    fn write(&self, data: &[u8]) -> Result<(), ()>; // TODO: Add actual results
    fn finalise(self) -> Result<&'a [u8], ()>; // TODO: Add more actual errors
}
