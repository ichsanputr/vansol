# Vansol

![Demo](https://i.ibb.co/7wRsHXw/demo.gif)

**Vansol** is a high-performance CLI tool for generating Solana vanity addresses using multithreaded search powered by Rayon in Rust.

## Features
- Generate Solana keypairs with custom prefixes, suffixes, or substrings in the public key
- Multithreaded for fast searching using the Rayon library
- Save generated keypairs to JSON files
- Customizable number of results and threads
- **Read default options from a `vansol.toml` configuration file**

## Benchmarks

| CPUs/Threads | Pattern        | Count | Time (approx) | Example Command                                 |
|--------------|---------------|-------|---------------|-------------------------------------------------|
| 8            | prefix: sol   | 1     | 1-2 sec       | `--prefix sol`                                  |
| 8            | contain: dev  | 5     | 5-10 sec      | `--contain dev --n 5`                           |
| 16           | prefix: test  | 10    | 3-8 sec       | `--prefix test --n 10 --threads 16`             |
| 4            | suffix: xyz   | 1     | <1 sec        | `--suffix xyz --threads 4`                      |

*Benchmarks are approximate and depend on CPU, pattern complexity, and randomness.*

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/ichsanputr/vansol.git
   cd vansol
   ```
2. Build with Cargo:
   ```sh
   cargo build --release
   ```
3. The binary will be in `target/release/vansol` (or `vansol.exe` on Windows).

### Install from Release Page

You can also download pre-built binaries for **Windows** and **Linux** from the [Releases page](https://github.com/ichsanputr/vansol/releases):

- Download the appropriate binary for your OS (e.g., `vansol.exe` for Windows, `vansol` for Linux)
- Place it in a directory in your `PATH`
- Make it executable on Linux: `chmod +x vansol`

## Usage

```sh
vansol -- [OPTIONS] 

```

If you clone this project.

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
vansol --prefix sol --suffix dev
```

Generate 5 addresses containing `test` using 8 threads:
```sh
vansol --contain test --n 5 --threads 8
```

## License
MIT 

## Recent Updates (v1.1.0, June 2024)

- **Configuration File Support:** You can now set default options in a `vansol.toml` file in your home or current directory. CLI arguments always override config file values.
- **Table Output:** Results and loaded config parameters are now displayed in a clean table format in the terminal for better readability.
- **Config Display:** If a config file is loaded, its parameters are shown in a table before the search begins.
- **Improved Spinner Handling:** Output is now clean and readable, with the progress spinner pausing automatically when printing tables or messages.
- **Dependencies Added:** `toml`, `dirs`, `serde`, and `prettytable` crates are now used for config parsing and table output. 