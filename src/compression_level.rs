#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum CompressionLevel {
    High,
    #[default]
    Med,
    Low,
    Custom(usize),
}

impl CompressionLevel {
    pub fn to_integer(&self) -> usize {
        match self {
            CompressionLevel::High => 10,
            CompressionLevel::Med => 6,
            CompressionLevel::Low => 2,
            CompressionLevel::Custom(x) => *x,
        }
    }
}
