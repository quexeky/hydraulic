# hydraulic
An interface and implementations of various compression algorithms.
# Installation 
## Binary
> :warning: This feature has not yet been added. See the 
> [CLI](https://github.com/quexeky/hydraulic/tree/cli) tree for the current status

## Library

> cargo install hydraulic 

## Example

```rust
use hydraulic::prelude::*;

fn compress(data: &[u8]) {
    // Create a new compressor which writes data to "foo.txt"
    let mut compressor = WriteEncoder::new(&Gzip {}, File::create("foo.txt").unwrap(), CompressionLevel::High);
    
    // Write "data" to the buffer
    compressor.write_all(&data).unwrap();
    
    // Compress and write the buffer to "foo.txt"
    compressor.flush().unwrap();
    
    // Finalises the compression and returns the file
    let finalised = compressor.finish().unwrap();

}
```