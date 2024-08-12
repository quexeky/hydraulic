use std::io::Write;
use crate::AlgorithmMeta;
use crate::write::types::WriteAlgorithm;

pub struct WriteDecoder<T: WriteAlgorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: Box<dyn Write>
}
