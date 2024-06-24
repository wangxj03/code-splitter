# code-splitter

[![Docs](https://docs.rs/code-splitter/badge.svg)](https://docs.rs/code-splitter/)
[![License](https://img.shields.io/crates/l/code-splitter)](https://github.com/wangxj03/code-splitter/blob/main/LICENSE.txt)
[![Crates.io](https://img.shields.io/crates/v/code-splitter)](https://crates.io/crates/code-splitter)

- **Rust Crate**: [code-splitter](https://crates.io/crates/code-splitter)
- **Python Bindings**: [code-splitter](https://pypi.org/project/code-splitter/)

## Introduction

Welcome to code-splitter, a Rust crate designed to split source code into chunks. It is particularly useful in Retrieval Augmented Generation (RAG), a technique that enhances the generation capabilities of Large Language Models (LLM) by leveraging external knowledge sources.

In a prototypical RAG application, longer documents are first split into chunks, encoded into embeddings, and then indexed into a vector database. When handling a new user query, the system first searches the vector database and retrieves the most relevant chunks based on their embeddings. These retrieved chunks are then used as context to augment the query, which is subsequently processed by an LLM. Chunking is essential for several reasons:

- Model Input Constraints: Embedding models, such as OpenAI's [text-embedding-3-small](https://platform.openai.com/docs/guides/embeddings/embedding-models), have a fixed maximum input token limit (e.g., 8192 tokens). Longer documents need to be divided into smaller chunks to fit within these input constraints.

- Precision and Relevance: By chunking longer documents, each chunk can be treated as a separate entity for retrieval. This improves the precision of the retrieval process, as smaller, focused chunks are more likely to be relevant to a specific query compared to an entire lengthy document.

- Improved Generation Quality: LLMs generate better responses when they have a concise and relevant context. Smaller chunks help in maintaining a clear and focused context, reducing the chances of irrelevant information diluting the response.

## Features

This crate utilizes [tree-sitter](https://crates.io/crates/tree-sitter) to parse code into an Abstract Syntax Tree (AST) and merge sibling nodes to create the largest possible chunks without exceeding the size limit.

It supports all languages that can be [parsed](https://tree-sitter.github.io/tree-sitter/#parsers) with tree-sitter, thanks to its extensible nature.

## Get Started

Add it to your project:

```sh
cargo add code-splitter
cargo add tree-sitter-<language>
```

### By Characters

```rust
use code_splitter::{CharCounter, Splitter};
use std::fs;

let max_chars = 1000;  // Maximum number of characters in a code chunk
let lang = tree_sitter_rust::language();  // Requires `cargo add tree-sitter-rust`
let splitter = Splitter::new(lang, CharCounter)
    .expect("Failed to load tree-sitter language")
    .with_max_size(max_chars);

let code = fs::read("path/to/code.rs").expect("Failed to read source code");
let chunks = splitter.split(&code);
```

### By Words

```rust
use code_splitter::{Splitter, WordCounter};
use std::fs;

let max_words = 200;  // Maximum number of words in a code chunk
let lang = tree_sitter_rust::language();  // Requires `cargo add tree-sitter-rust`
let splitter = Splitter::new(lang, WordCounter)
    .expect("Failed to load tree-sitter language")
    .with_max_size(max_words);

let code = fs::read("path/to/code.rs").expect("Failed to read source code");
let chunks = splitter.split(&code);
```

### By Tokens with Huggingface

Requires the `tokenizers` feature to be activated.

```sh
cargo add code-splitter --features tokenizers
```

```rust
use code_splitter::Splitter;
use std::fs;
use tokenizers::Tokenizer;

let max_tokens = 500;  // Maximum number of tokens in a code chunk
let lang = tree_sitter_rust::language();  // Requires `cargo add tree-sitter-rust`
let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)
    .expect("Failed to load tokenizer");
let splitter = Splitter::new(lang, tokenizer)
    .expect("Failed to load tree-sitter language")
    .with_max_size(max_tokens);

let code = fs::read("path/to/code.rs").expect("Failed to read source code");
let chunks = splitter.split(&code);
```

### By Tokens with Tiktoken

Requires the `tiktoken-rs` feature to be activated.

```sh
cargo add code-splitter --features tiktoken-rs
```

```rust
use code_splitter::Splitter;
use std::fs;
use tiktoken_rs::cl100k_base;

let max_tokens = 500;  // Maximum number of tokens in a code chunk
let lang = tree_sitter_rust::language();  // Requires `cargo add tree-sitter-rust`
let bpe = cl100k_base().expect("Failed to load tokenizer");
let splitter = Splitter::new(lang, bpe)
    .expect("Failed to load tree-sitter language")
    .with_max_size(max_tokens);

let code = fs::read("path/to/code.rs").expect("Failed to read source code");
let chunks = splitter.split(&code).unwrap();
```

## Inspiration

This crate was inspired by LlamaIndex's [CodeSplitter](https://docs.llamaindex.ai/en/v0.10.19/api/llama_index.core.node_parser.CodeSplitter.html) which, in turn, was based on SweepAI's blog [post](https://docs.sweep.dev/blogs/chunking-2m-files).

The project setup was inspired by [text-splitter](https://github.com/benbrandt/text-splitter), but the splitting algorithm was developed independently.
