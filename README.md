# hydraulic
An interface and implementations of various compression algorithms. 

## Overview
As it is currently, this library is simply a plain interface without any implementations. 
This will be changed in future versions

## Usage 
### Binary
> :warning: This feature has not yet been added. See the 
> [CLI](https://github.com/quexeky/hydraulic/tree/cli) branch for the current status

### Library

hydraulic may be installed with the command:

> cargo install hydraulic

Or by adding the following to your Cargo.toml

```toml
[dependencies]
hydraulic = { version = "0.1.0", features = ["full"]}
```

## Example

```rust
use hydraulic::prelude::*;

fn compress(data: &[u8]) {
    
    let alg = Gzip::new();
    // Create a new compressor which writes data to "foo.txt"
    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::High);
    
    // Send "data" to the buffer
    compressor.queue(&data).unwrap();
    
    // Compress and write the buffer to "foo.txt"
    compressor.write().unwrap();
    
    // Finalises the compression and returns the file where data is written
    let finalised = compressor.finish().unwrap();

}
```