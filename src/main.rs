use std::fs;
use hydraulic::CompressionLevel;
use hydraulic::gz::Gzip;
use hydraulic::write::decompress::WriteDecoder;
use hydraulic::write::compress::WriteEncoder;

fn main() {
    let data = fs::read_to_string("foo.txt").unwrap();
    println!("{:?}", data.as_bytes());
    let mut compressor = WriteEncoder::new(&Gzip {}, CompressionLevel::Low);
    compressor.write_all(data.as_bytes()).unwrap();
    compressor.flush().unwrap();
    let finalised = compressor.finish().unwrap();
    println!("{:?}", finalised);
    let mut decompressor = WriteDecoder::new(&Gzip {});
    decompressor.write_all(&*finalised).unwrap();
    decompressor.flush().unwrap();
    let decompressed = decompressor.finish().unwrap();
    println!("{:?}", decompressed);
}