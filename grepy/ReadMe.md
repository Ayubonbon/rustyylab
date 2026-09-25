# Grepy

Grepy is a small grep-inspired command-line tool written in Rust.
I built it mainly to understand how file reading, buffered input, command parsing, and simple CLI tools work in Rust without relying on external crates.
At the moment, Grepy can search for a word inside a file and supports a few basic search flags.

## Features

* Search for a word inside a text file
* Use files from anywhere in the system by providing the file path
* Ignore uppercase and lowercase differences
* Show matching line numbers
* Invert the search and show non-matching lines
* Simple interactive `grepy>` prompt
* No external dependencies

## Usage

Clone the repository:

```bash
git clone <your-repository-url>
cd grepy
```
Run the project:

```bash
cargo run
```
Grepy will display the banner, available options, and then wait for your search command.

Basic syntax:

```text
grepy> <file-path> <word> [flags]
```
Example:
```text
grepy> input.txt rust
```
You can also provide the full path of a file:
```text
grepy> /home/user/Documents/notes.txt rust
```
## Available Flags

| Flag | Description                                    |
| ---- | ---------------------------------------------- |
| `-i` | Ignore uppercase and lowercase differences     |
| `-n` | Show the line number of each result            |
| `-v` | Show lines that do not contain the search word |

Flags can also be combined:
```text
grepy> input.txt rust -i -n
```
This searches for `rust` without caring about uppercase/lowercase differences and prints the matching line numbers.
Another example:
```text
grepy> logs.txt error -v
```
This prints every line that does **not** contain `error`.

## How It Works

Grepy uses Rust's standard library to:

1. Read the command entered after the `grepy>` prompt
2. Split it into the file path, search word, and optional flags
3. Open the file using `std::fs::File`
4. Read the file line by line using `BufReader`
5. Print the lines that match the selected search behaviour

The project is intentionally kept simple for now so I can build the functionality myself and understand what is happening instead of hiding everything behind a CLI library.

## Project Structure

```text
grepy/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```
