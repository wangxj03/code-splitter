use code_splitter::{CharCounter, Splitter, WordCounter};
use std::fs;

#[test]
fn split_by_chars() {
    let code = fs::read("tests/testdata/markdown.md").unwrap();
    let lang = tree_sitter_md::language();

    let splitter = Splitter::new(lang, CharCounter).unwrap().with_max_size(200);
    let chunks = splitter.split(&code).unwrap();
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}

#[test]
fn split_by_tokens_huggingface() {
    use tokenizers::tokenizer::Tokenizer;

    let code = fs::read("tests/testdata/markdown.md").unwrap();
    let lang = tree_sitter_md::language();

    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();

    let splitter = Splitter::new(lang, tokenizer).unwrap().with_max_size(100);
    let chunks = splitter.split(&code).unwrap();
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}

#[test]
fn split_by_tokens_tiktoken() {
    use tiktoken_rs::cl100k_base;

    let code = fs::read("tests/testdata/markdown.md").unwrap();
    let lang = tree_sitter_md::language();
    let bpe = cl100k_base().unwrap();

    let splitter = Splitter::new(lang, bpe).unwrap().with_max_size(100);
    let chunks = splitter.split(&code).unwrap();
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}

#[test]
fn split_by_words() {
    let code = fs::read("tests/testdata/markdown.md").unwrap();
    let lang = tree_sitter_md::language();

    let splitter = Splitter::new(lang, WordCounter).unwrap().with_max_size(50);
    let chunks = splitter.split(&code).unwrap();
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}
