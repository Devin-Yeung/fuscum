# Fuscum

[![Built with devenv][devenv-badge]][devenv-url]
[![Built with Nix][nix-badge]][nix-url]
[![Build Status][actions-badge]][actions-url]
[![MIT Licensed][mit-badge]][mit-url]

[devenv-badge]: https://devenv.sh/assets/devenv-badge.svg

[devenv-url]: https://devenv.sh

[nix-badge]: https://img.shields.io/static/v1?logo=nixos&logoColor=white&label=&message=Built%20with%20Nix&color=41439a

[nix-url]: https://builtwithnix.org

[actions-badge]: https://github.com/Devin-Yeung/fuscum/actions/workflows/ci.yml/badge.svg?branch=master

[actions-url]: https://github.com/Devin-Yeung/fuscum/actions/workflows/ci.yml

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[mit-url]: https://github.com/Devin-Yeung/fuscum/blob/master/LICENSE-MIT

A Rust implementation of MOSS (Measure Of Software Similarity) for detecting code plagiarism through document fingerprinting using the winnowing algorithm.

## Features

- **Fast**: Parallel processing with Rayon, efficient rolling hash (O(1) per window)
- **Language-agnostic preprocessing**: Supports 9 programming languages
- **Multiple output formats**: Console table, JSON, HTML network visualization
- **Configurable**: Adjustable k-gram size, window size, similarity threshold

## Supported Languages

C, C++, Go, Java, JavaScript, Python, Ruby, Rust, TypeScript

## Installation

```bash
cargo install --git https://github.com/Devin-Yeung/fuscum
```

Or build from source:

```bash
cargo build --release
```

## Usage

```bash
# Basic usage
fuscum-cli --dir ./src --pat "**/*.py" --lang Python

# With output files
fuscum-cli --dir ./src --pat "**/*.rs" --lang Rust --json results.json --network network.html

# Configure detection sensitivity
fuscum-cli --dir ./src --pat "**/*.js" --lang JavaScript --kgram 30 --window 50 --threshold 0.4
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--dir` | Directory to scan | required |
| `--pat` | Glob pattern for files | required |
| `--lang` | Language for preprocessing | required |
| `--kgram` | K-gram size (characters) | 35 |
| `--window` | Window size for winnowing | 40 |
| `--threshold` | Minimum similarity (0-1) | 0.5 |
| `--top` | Top-K matches per file | 5 |
| `--json` | Export results to JSON | - |
| `--network` | Generate HTML visualization | - |

## How It Works

1. **Preprocessing**: Parse source code with AST, remove comments, normalize identifiers and strings
2. **K-gram generation**: Split preprocessed code into overlapping k-length windows, hash each using Rabin-Karp rolling hash
3. **Winnowing**: Select representative hashes from each sliding window (rightmost minimum)
4. **Similarity**: Compute pairwise similarity as `|intersection| / |base fingerprint|`

## Name

Derived from [*Sphagnum fuscum*](https://en.wikipedia.org/wiki/Sphagnum_fuscum) (rusty peat moss), a nod to the original MOSS system and Rust.

## References

Schleimer, S., Wilkerson, D. S., & Aiken, A. (2003). Winnowing: local algorithms for document fingerprinting. *Proceedings of the 2003 ACM SIGMOD*, 76–85. https://doi.org/10.1145/872757.872770

## License

MIT OR Apache-2.0
