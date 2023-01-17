# 🦀 Duplicate finder

This small tool finds duplicate files in any directory specified using the extremely fast [xxHash](https://github.com/Cyan4973/xxHash) (64) hashing algorithm

It creates a new hashmap with xxHash set as the hashing fucntion, then tries to insert every file into it. On collision, the file is marked as duplicate and added to a deletion list. Once the scan is complete, the duplicate files are deleted.

### Performance

> Benchmarks are done with a release build and may be slower on debug/dev.

Scanning my entire picture library of 18GB (~14k files) took ~22 seconds.

A smaller folder of 3.2GB (~4k files) took ~4 seconds.

So it's speed is slightly under 1Gbps.

## Installation

Grab the latest binaries for your operating system from the [release](https://github.com/joinemm/duplicate-finder/releases) page, or build from source using `cargo build --release`, compiled binaries will be located in `./target/release/`.

## Usage

```bash
# get help
duplicate-finder --help

# dry run
duplicate-finder <path> -d

# deletes any duplicates
duplicate-finder <path>
```
