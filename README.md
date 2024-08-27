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

```
cargo install hydraulic
```

Or by adding the following to your Cargo.toml

```toml
[dependencies]
hydraulic = { version = "0.1.0", features = ["full"]}
```

### Examples
A simple compression function using already implemented Gzip compression.

Ensure that the "gzip" feature is enabled

```toml
[dependencies]
hydraulic = { version = "0.1.0", features = ["gzip"]}
```
Then, in main.rs:
```rust
use std::fs::File;
use std::io::Read;
use hydraulic::Gzip;
use hydraulic::prelude::*;

fn compress(data: &[u8]) -> File {

    let alg = Gzip::new();
    // Create a new compressor which writes data to "foo.txt"
    let mut compressor = WriteEncoder::new(&alg, File::create("foo.txt").unwrap(), CompressionLevel::High);

    // Send "data" to the buffer
    compressor.queue(&data);

    // Compress and write the buffer to "foo.txt"
    compressor.write().unwrap();

    // Finalises the compression and returns the file where data is written
    let finalised = compressor.finish().unwrap();

    finalised
}
```

Alternatively, you may implement your own compression algorithm through the provided traits:

```rust
use crate::algorithm_meta::AlgorithmMeta;
use crate::algorithms::Algorithm;
use crate::errors::compression_error::CompressionError;
use crate::errors::decompression_error::DecompressionError;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Gzip {}
impl Gzip {
    pub fn new() -> Self {
        Self {}
    }
}
impl Algorithm for Gzip {
    fn finalise_encode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, CompressionError> {
        // Final encoding steps go here
    }

    fn finalise_decode(&self, meta: &AlgorithmMeta) -> Result<Vec<u8>, DecompressionError> {
        // Final decoding steps go here
    }
    
    fn partial_encode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, CompressionError> {
        // Implement your decompression of a single data block here
    }

    fn partial_decode(
        &self,
        data: &[u8],
        meta: &AlgorithmMeta,
    ) -> Result<Vec<u8>, DecompressionError> {
        // Implement your compression of a single data block here
    }
}
```

More examples of usage can be found [here](https://github.com/quexeky/hydraulic/tree/main/examples)

## Contributing
This project is very small and only maintained by @[quexeky](https://github.com/quexeky). Contributions are greatly
appreciated. A contribution guide will be added later on