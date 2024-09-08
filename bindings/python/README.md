# code-splitter Python Bindings

[![License](https://img.shields.io/crates/l/code-splitter)](https://github.com/wangxj03/code-splitter/blob/main/LICENSE)
[![PyPI](https://img.shields.io/pypi/v/code-splitter)](https://pypi.org/project/code-splitter/)

The `code-splitter` Python package provides bindings for the [code-splitter](https://crates.io/crates/code-splitter) Rust crate. It leverages the [tree-sitter](https://tree-sitter.github.io/tree-sitter/) parsing library and tokenizers to split code into semantically meaningful chunks. This functionality is particularly useful in Retrieval Augmented Generation (RAG), a technique that enhances the generation capabilities of Large Language Models (LLMs) by leveraging external knowledge sources.

## Installation

You can install the package from PyPI:

```shell
pip install code-splitter
```

## Usage

Here's an example of how to use the package:

```python
from code_splitter import Language, CharSplitter

# Load the code you want to split
with open("example.py", "rb") as f:
    code = f.read()

# Create a splitter instance
splitter = CharSplitter(Language.Python, max_size=200)

# Split the code into chunks
chunks = splitter.split(code)

# Print the chunks
for chunk in chunks:
    print(f"Start: {chunk.start}, End: {chunk.end}, Size: {chunk.size}")
    print(chunk.text)
    print()
```

This example uses the `CharSplitter` to split Python code into chunks of maximum 200 characters. The `Chunk` objects contain information about the start and end lines, size, and the actual text of the chunk.

### Available Splitters

The package provides the following splitters:

- `CharSplitter`: Splits code based on character count.
- `WordSplitter`: Splits code based on word count.
- `TiktokenSplitter`: Splits code based on Tiktoken tokenizer.
- `HuggingfaceSplitter`: Splits code based on HuggingFace tokenizers.

### Supported Languages

The following programming languages are currently supported:

- Golang
- Markdown
- Python
- Rust

## Examples

Here are some examples of splitting code using different splitters and languages:

### Split Python Code by Characters

```python
from code_splitter import Language, CharSplitter

splitter = CharSplitter(Language.Python, max_size=200)
chunks = splitter.split(code)
```

### Split Markdown by Words

```python
from code_splitter import Language, WordSplitter

splitter = WordSplitter(Language.Markdown, max_size=50)
chunks = splitter.split(code)
```

### Split Rust Code by Tiktoken Tokenizer

```python
from code_splitter import Language, TiktokenSplitter

splitter = TiktokenSplitter(Language.Rust, max_size=100)
chunks = splitter.split(code)
```

### Split Go Code by HuggingFace Tokenizer

```python
from code_splitter import Language, HuggingfaceSplitter

splitter = HuggingfaceSplitter(Language.Golang, max_size=100, pretrained_model_name_or_path="bert-base-cased")
chunks = splitter.split(code)
```

For more examples, please refer to the [tests](https://github.com/wangxj03/code-splitter/tree/main/bindings/python/tests) directory in the repository.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
