use std::io::{Read};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;

pub struct ReadDecoder<T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: Box<dyn Read>
}
