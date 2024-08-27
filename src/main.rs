use std::fs::File;
use hydraulic::Gzip;
use hydraulic::prelude::ReadEncoder;

fn main() {
    let mut compressor = ReadEncoder::new(&Gzip {}, File::open("foo.txt").unwrap());
    let mut buf = [0; 32];
    let decompressed = compressor.read(&mut buf).unwrap();
}