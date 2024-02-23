# Compression App

`compression_app` is a simple command-line tool written in Rust for compressing files into gzip format.

## Features

- Compresses files into gzip format.
- Allows specifying the source file and target location via command-line arguments.
- Measures the time taken for compression.

## Getting Started

### Prerequisites

Before running this application, ensure you have Rust and Cargo installed. You can install them by following the instructions at [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/compression_app.git
cd compression_app
```

2. Build the application:

```bash
cargo build --release
```

### Usage

```bash
compression_app <source_file> <target_location>
```

Replace `<source_file>` with the path to the original file you want to compress, and `<target_location>` with the path where you want to save the compressed file.

### Example

```bash
compression_app /path/to/original/file.txt /path/to/compressed/file.txt.gz
```

This command will compress the file `file.txt` located at `/path/to/original/` directory and save the compressed file as `file.txt.gz` in the `/path/to/compressed/` directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This application utilizes the `flate2` crate for compressing files into gzip format in Rust.
- Special thanks to the Rust community for their contributions and support.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
