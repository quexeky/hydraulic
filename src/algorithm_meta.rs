use crate::compression_level::CompressionLevel;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct AlgorithmMeta {
    pub(crate) level: Option<CompressionLevel>,
}
