use crate::features::gz::Gzip;
use crate::prelude::ReadDecoder;
use crate::tests::util::generate_random_bytes;
use crate::write::compress::WriteEncoder;
use crate::CompressionLevel;
use std::fs::File;

#[test]
fn write_read_high() {
    let data = generate_random_bytes();
    
    let alg = Gzip::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::High);
    compressor.queue(&data);
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();
    
    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_med() {
    let data = generate_random_bytes();
    
    let alg = Gzip::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::Med);
    compressor.queue(&data);
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();

    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_low() {
    let data = generate_random_bytes();

    let alg = Gzip::new();
    
    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::Low);
    compressor.queue(&data);
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();

    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
