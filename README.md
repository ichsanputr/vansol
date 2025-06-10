# vansol

**vansol** is a high-performance CLI tool for generating Solana vanity addresses using multithreaded search powered by Rayon in Rust.

## Features
- Generate Solana keypairs with custom prefixes, suffixes, or substrings in the public key
- Multithreaded for fast searching using the Rayon library
- Save generated keypairs to JSON files
- Customizable number of results and threads

## Installation

1. Clone the repository:
   ```sh
   git clone <your-repo-url>
   cd vansol
   ```
2. Build with Cargo:
   ```sh
   cargo build --release
   ```
3. The binary will be in `target/release/vansol` (or `vansol.exe` on Windows).

## Usage

```sh
cargo run -- [OPTIONS]
```
Or, after building:
```sh
./target/release/vansol [OPTIONS]
```

### Options
- `-p, --prefix <PREFIX>`: Specify a prefix for the public key
- `-s, --suffix <SUFFIX>`: Specify a suffix for the public key
- `-c, --contain <CONTAIN>`: Specify a substring to be contained in the public key
- `-n, --n <N>`: Number of addresses to generate (default: 1)
- `-t, --threads <THREADS>`: Number of threads to use (default: number of CPU cores)

### Example
Generate a Solana address starting with `sol` and ending with `dev`:
```sh
cargo run -- --prefix sol --suffix dev
```

Generate 5 addresses containing `test` using 8 threads:
```sh
cargo run -- --contain test --n 5 --threads 8
```

## License
MIT 