:exclamation: Deprecated: see https://github.com/huggingface/tokenizers

# tokenize`rs`

[![Build Status](https://travis-ci.org/epwalsh/tokenizers.svg?branch=master)](https://travis-ci.org/epwalsh/tokenizers)
[![Latest version](https://img.shields.io/crates/v/tokenizers.svg)](https://crates.io/crates/tokenizers)
![License](https://img.shields.io/crates/l/tokenizers.svg)

Provides a generic `Tokenizer` trait that makes it easy to implement a custom, *fast* tokenizer. The `Tokenizer` builds on a `Token` struct that encapsulates a string slice with a copy-on-write implementation, meaning a new string slice is only creating when the original term is modified.

Implementing the `Tokenizer` trait requires a single function, `tokenize`, which should return an iterator of `Token`s.

## Examples

The [examples/cli.rs](./examples/cli.rs) shows how a simple whitespace tokenizer can be implemented and wrapped in a nice command-line interface. You can run the example on a small text file [examples/poem.txt](./examples/poem.txt) with

```
cargo run --example cli -- examples/poem.txt
```

> Due to its simplicity, the whitespace tokenizer is actually quite fast. It can process over 40 million tokens per second in memory and over 10 million tokens per second from disk to disk through the CLI (done on a desktop running Ubuntu 18.04, with a 16GB Intel Core i5-3570K CPU @ 3.40GHz).
