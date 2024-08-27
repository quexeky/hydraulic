use crate::features::gz::Gzip;
use crate::tests::util::generate_random_bytes;
use crate::write::compress::WriteEncoder;
use crate::write::decompress::WriteDecoder;
use crate::CompressionLevel;

#[test]
fn write_write_high() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, CompressionLevel::High);
    compressor.write_all(&data).unwrap();
    compressor.flush().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = WriteDecoder::new(&Gzip {});
    decompressor.write_all(&*finalised).unwrap();
    decompressor.flush().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_write_med() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, CompressionLevel::Med);
    compressor.write_all(&data).unwrap();
    compressor.flush().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = WriteDecoder::new(&Gzip {});
    decompressor.write_all(&*finalised).unwrap();
    decompressor.flush().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
#[test]
fn write_write_low() {
    let data = generate_random_bytes();

    let mut compressor = WriteEncoder::new(&Gzip {}, CompressionLevel::Low);
    compressor.write_all(&data).unwrap();
    compressor.flush().unwrap();
    let finalised = compressor.finish().unwrap();

    let mut decompressor = WriteDecoder::new(&Gzip {});
    decompressor.write_all(&*finalised).unwrap();
    decompressor.flush().unwrap();
    let decompressed = decompressor.finish().unwrap();

    assert_eq!(data, &*decompressed)
}
