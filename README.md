# rg-like

A ripgrep-like CLI tool written in Rust. Searches for a word in a file and displays matching lines with the matched word highlighted in color.

## Usage

```
rg-like <word> <file>
```

## Example

```
rg-like hello hello.txt
```

Output:
```
1: say hello world
3: hello from the other side
```

The matched word (`hello`) is highlighted in **red bold**.

## How it works

1. Reads the file into memory
2. Splits it into lines
3. Builds a `HashMap<String, Vec<usize>>` mapping every word to the line numbers it appears on
4. Filters the map for the search word
5. Prints each matching line with the search word highlighted

## Project structure

```
src/
  main.rs            - CLI argument parsing, file reading, orchestration
  search/
    mod.rs           - search() and sentence() functions
    words.rs         - hash() function that builds the word-to-lines map
```

## Dependencies

- [`colored`](https://crates.io/crates/colored) - Terminal text coloring

## Build and run

```
cargo build --release
cargo run -- <word> <file>
```
