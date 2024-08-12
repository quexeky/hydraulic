use std::io::{Write};
use crate::AlgorithmMeta;
use crate::write::types::WriteAlgorithm;

pub struct WriteEncoder<T: WriteAlgorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: Box<dyn Write>
}
