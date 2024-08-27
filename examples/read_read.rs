use std::fs::File;
use std::io::Read;
use hydraulic::Gzip;
use hydraulic::prelude::*;

fn main() {
    let data = vec![2u8; 32];

    let f = compress(&*data);

    println!("{:?}", f.bytes());
}

fn compress(data: &[u8]) -> File {

    // Create a new Gzip class with default values
    let alg = Gzip::default();

    // Create a new compressor which writes data to "foo.txt"
    let mut compressor = WriteEncoder::new(&alg, File::create("../../foo.txt").unwrap(), CompressionLevel::High);

    // Send "data" to the buffer
    compressor.queue(&data);

    // Compress and write the buffer to "foo.txt"
    compressor.write().unwrap();

    // Finalises the compression and returns the file where data is written
    let finalised = compressor.finish().unwrap();

    finalised
}