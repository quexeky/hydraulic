use std::io::{BufReader};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;

pub struct ReadEncoder<T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: BufReader<Vec<u8>>
}
