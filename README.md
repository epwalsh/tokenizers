# tokenize`rs`

[![Build Status](https://travis-ci.org/epwalsh/tokenizers.svg?branch=master)](https://travis-ci.org/epwalsh/tokenizers)
[![Latest version](https://img.shields.io/crates/v/tokenizers.svg)](https://crates.io/crates/tokenizers)
![License](https://img.shields.io/crates/l/tokenizers.svg)

Lightweight, easily customizable tokenizers for Rust with a CLI.

## Features
- A generic `Tokenizer` trait that makes it easy to implement a custom, *fast* tokenizer. The `Tokenizer` builds on a `Token` type which is copy-on-write, meaning a new string is only creating when the original term is modified. This is what makes it so fast. 
- A default whitespace tokenizer that can process over 10 million tokens per second from disk to disk through the CLI, and over 40 million tokens per second in memory.*
> \* Benchmarks done on a desktop running Ubuntu 18.04, with a 16GB Intel Core i5-3570K CPU @ 3.40GHz.
