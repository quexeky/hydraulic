use std::fs::File;
use crate::features::gz::Gzip;
use crate::tests::util::generate_random_bytes;
use crate::write::compress::WriteEncoder;
use crate::write::decompress::WriteDecoder;
use crate::CompressionLevel;
use crate::prelude::ReadDecoder;

#[test]
fn write_read_high() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, File::create("foo.txt").unwrap(), CompressionLevel::High);
    compressor.write_all(&data).unwrap();
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = ReadDecoder::new(&Gzip {}, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_med() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, File::create("foo.txt").unwrap(), CompressionLevel::Med);
    compressor.write_all(&data).unwrap();
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = ReadDecoder::new(&Gzip {}, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_read_low() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, File::create("foo.txt").unwrap(), CompressionLevel::Low);
    compressor.write_all(&data).unwrap();
    compressor.write().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = ReadDecoder::new(&Gzip {}, finalised);
    decompressor.read_all().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
