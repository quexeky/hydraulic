use std::io::{Read};
use crate::AlgorithmMeta;
use crate::read::types::ReadAlgorithm;

pub struct ReadEncoder<T: ReadAlgorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: Box<dyn Read>
}
