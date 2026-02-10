# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Fuscum is a Rust implementation of MOSS (Measure Of Software Similarity), which detects code plagiarism through document fingerprinting using the winnowing algorithm. The project implements the research from "Winnowing: local algorithms for document fingerprinting" by Schleimer, Wilkerson, and Aiken (SIGMOD 2003).

## Architecture

The workspace consists of two crates:

- **crates/fuscum** - Core library containing the fingerprinting algorithm
- **crates/fuscum-cli** - Command-line interface for running similarity detection

### Core Algorithm Flow

The fingerprinting pipeline follows these stages:

1. **Preprocessing** (`crates/fuscum/src/preprocess/`):
   - Uses ast-grep to parse source code into an AST
   - Removes comments, normalizes identifiers (e.g., variable names → "v"), and normalizes string literals
   - Removes all whitespace from the preprocessed source
   - Supports 9 languages: Python, C, C++, JavaScript, TypeScript, Java, Go, Rust, Ruby

2. **K-gram Generation** (`crates/fuscum/src/kgram.rs`):
   - Splits preprocessed bytes into overlapping k-length windows
   - Computes hash for each window using Rabin-Karp rolling hash
   - Default k-gram size: 35 characters

3. **Winnowing** (`crates/fuscum/src/winnow.rs`):
   - Selects representative hashes from the k-gram sequence using a sliding window
   - For each window, selects the rightmost minimum hash value
   - Default window size: 40

4. **Fingerprinting** (`crates/fuscum/src/fingerprint.rs`):
   - Stores selected hashes as a set (the fingerprint)
   - Similarity between two files = |intersection| / |base fingerprint|

### CLI Architecture

`crates/fuscum-cli/src/main.rs` orchestrates:

1. **File Discovery** (`discovery.rs`): Uses glob patterns to find source files
2. **Fingerprint Generation**: Parallel processing using Rayon
3. **Similarity Analysis** (`analysis.rs`): Compares all pairs, filters by threshold, keeps top-K matches per file
4. **Output** (`output.rs`, `visual.rs`): Table output, JSON export, HTML network visualization

## Common Commands

### Build and Run
```bash
cargo build --release
cargo run --bin fuscum-cli -- --dir . --pat "**/*.py" --lang Python --json results.json --network network.html
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p fuscum

# Run a single test
cargo test -p fuscum test_name

# Run benchmarks
cargo bench -p fuscum
```

### Linting
```bash
cargo clippy --all-targets --all-features
```

### Formatting
```bash
cargo fmt
```

## Key Implementation Details

### Rolling Hash Configuration

The Rabin-Karp rolling hash uses:
- Base (B): 257
- Modulus (M): u64::MAX (effectively 2^64, using natural overflow)
- Defined in `crates/fuscum/src/hash/rabin_karp.rs` as `RabinKarp<B, M>`

The default k-gram generator is `RollingHashKgram<257, { u64::MAX }>`, which provides O(1) hash computation per window compared to O(k) for standard hashing.

### Preprocessor Customization

Language-specific preprocessors in `crates/fuscum/src/preprocess/lang.rs` support:
- Variable substitution (`subst_var`): defaults to "v"
- String substitution (`subst_string`): defaults to "\"s\""
- Comment removal (`remove_comments`): defaults to true

These use the `typed_builder` crate for builder pattern configuration.

### AST Operations

Preprocessing uses ast-grep's tree editing API (`crates/fuscum/src/preprocess/tree.rs`):
- `remove_comments()`: Removes comment nodes
- `subst_ident()`: Replaces identifier nodes with unified representation
- `subst_string()`: Replaces string literals with unified representation

## Adding Support for New Languages

To add a new language:

1. Add a variant to `Lang` enum in `crates/fuscum-cli/src/arg.rs`
2. Add `impl_lang_preprocessor!` macro invocation in `crates/fuscum/src/preprocess/lang.rs` with:
   - The ast-grep language constant
   - The AST node kind for identifiers
   - The AST node kind for strings
   - The AST node kind for comments
3. Export the preprocessor type in `crates/fuscum/src/preprocess/mod.rs`
4. Add a test fixture in `fixtures/langs/`

## Similarity Scoring

The CLI computes pairwise similarity as `intersection(base, target) / |base|`. This is asymmetric - file A may be 90% similar to file B, but file B may only be 50% similar to file A if file A is longer. The output shows the base file's perspective.
