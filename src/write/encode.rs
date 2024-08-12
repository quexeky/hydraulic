use std::io::{Write};
use crate::AlgorithmMeta;
use crate::algorithms::Algorithm;

pub struct WriteEncoder<T: Algorithm> {
    meta: AlgorithmMeta,
    encoder: T,
    data: Box<dyn Write>
}
