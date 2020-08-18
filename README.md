# 🦀 Duplicate finder

This small tool finds duplicate files in any directory specified by hashing every file using xxHash then trying to insert it into a hashmap. On collision, the file is marked as duplicate. It's a naive approach, unoptimized, but still not too slow. Managed to scan my 5814 image files (~5Gb) in `5.443822s`, so speed is around 1Gbps (when built with `cargo build --release`)

## Installation:

#### Windows:
Grab the .exe latest binaries from the [releases](https://github.com/joinemm/duplicate-finder/releases) page

#### Other:
Build from source using `cargo build --release`

## Usage:

```bash
# get help
duplicate-finder --help

# dry run
duplicate-finder <path> -d

# deletes any duplicates
duplicate-finder <path>
```
