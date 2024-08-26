mod args;

use std::fs;
use std::fs::File;
use hydraulic::Gzip;

use hydraulic::prelude::*;
use clap;

fn main() {
    let data = fs::read_to_string("../../foo.txt").unwrap();
    println!("{:?}", data.as_bytes());
    let dest = File::create_new("hydraulic").unwrap();
    let mut compressor = WriteEncoder::new(&Gzip {}, dest, CompressionLevel::High);
    compressor.write_all(data.as_bytes()).unwrap();
    compressor.flush().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = ReadDecoder::new(&Gzip {}, finalised);
    let mut data = decompressor.read_all().unwrap();
    data.append(&mut decompressor.finish().unwrap());
    println!("{:?}", data);
}
