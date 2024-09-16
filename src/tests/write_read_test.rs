use crate::prelude::ReadDecoder;
use crate::tests::util::generate_random_bytes;
use crate::write::compress::WriteEncoder;
use crate::CompressionLevel;
use std::fs::File;
use serial_test::serial;
use crate::features::no_compression::NoCompression;

#[test]
#[serial]
fn write_read_high() {
    let data = generate_random_bytes();

    let alg = NoCompression::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::High);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let file = File::open("foo.txt").unwrap();

    let mut decompressor = ReadDecoder::new(&alg, file);

    let mut decompressed = decompressor.read_all().unwrap();
    decompressed.append(&mut decompressor.finish().unwrap());

    assert_eq!(data, &*decompressed)
}
#[test]
#[serial]
fn write_read_med() {
    let data = generate_random_bytes();

    let alg = NoCompression::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::Med);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let file = File::open("foo.txt").unwrap();

    let mut decompressor = ReadDecoder::new(&alg, file);

    let mut decompressed = decompressor.read_all().unwrap();
    decompressed.append(&mut decompressor.finish().unwrap());

    assert_eq!(data, &*decompressed)
}
#[test]
#[serial]
fn write_read_low() {
    let data = generate_random_bytes();

    let alg = NoCompression::new();
    
    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::Low);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let file = File::open("foo.txt").unwrap();

    let mut decompressor = ReadDecoder::new(&alg, file);

    let mut decompressed = decompressor.read_all().unwrap();
    decompressed.append(&mut decompressor.finish().unwrap());

    assert_eq!(data, &*decompressed)
}
