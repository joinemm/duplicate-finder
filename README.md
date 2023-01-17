# 🦀 Duplicate finder

This small tool finds duplicate files in any directory specified by hashing every file using xxHash then trying to insert it into a hashmap. On collision, the file is marked as duplicate and added to a deletion list. Once the scan is complete, the duplicate files are deleted.

### Performance

> Benchmarks are done with a release build and may be slower on debug/dev.

Scanning my entire picture library of 18GB (~14k files) took ~22 seconds.

A smaller folder of 3.2GB (~4k files) took ~4 seconds.

So it's speed is slightly under 1Gbps.

## Installation

#### Windows
Grab the .exe latest binaries from the [releases](https://github.com/joinemm/duplicate-finder/releases) page

#### Linux
Build from source using `cargo build --release`

Binaries will be located in `./target/release/`

## Usage

```bash
# get help
duplicate-finder --help

# dry run
duplicate-finder <path> -d

# deletes any duplicates
duplicate-finder <path>
```
