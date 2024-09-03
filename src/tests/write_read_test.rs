use crate::features::gz::Gzip;
use crate::prelude::ReadDecoder;
use crate::tests::util::generate_random_bytes;
use crate::write::compress::WriteEncoder;
use crate::CompressionLevel;
use std::fs::File;
use std::io::Read;

#[test]
fn write_read_high() {
    let data = generate_random_bytes();

    println!("Original Data: {:?}", data);
    
    let alg = Gzip::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("../../foo.tar.gz").unwrap(), CompressionLevel::High);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let mut f = File::open("../../foo.tar.gz").unwrap();
    let mut b = vec![];
    println!("{:?}", f.read_to_end(&mut b));
    println!("Bytes: {:?}", b);

    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, f);
    let mut decompressed = decompressor.read_all().unwrap();
    println!("Decompressed #1:{:?}", &decompressed);
    decompressed.extend(decompressor.finish().unwrap());
    println!("Decompressed #2: {:?}", &decompressed);

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_med() {
    let data = generate_random_bytes();
    
    let alg = Gzip::new();

    let mut compressor = WriteEncoder::new(&alg, File::create("../../foo.tar.gz").unwrap(), CompressionLevel::Med);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, File::open("../../foo.tar.gz").unwrap());
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_low() {
    let data = generate_random_bytes();

    let alg = Gzip::new();
    
    let mut compressor = WriteEncoder::new(&alg, File::create("../../foo.tar.gz").unwrap(), CompressionLevel::Low);
    compressor.queue(&data);
    compressor.write().unwrap();
    compressor.finish().unwrap();

    let alg = Gzip::new();

    let mut decompressor = ReadDecoder::new(&alg, File::open("../../foo.tar.gz").unwrap());
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
