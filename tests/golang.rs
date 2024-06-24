use code_splitter::{CharCounter, Sizer, Splitter, WordCounter};
use std::fs;

const TEST_FILE: &str = "tests/testdata/greetings.go";

fn read_test_file() -> Vec<u8> {
    fs::read(TEST_FILE).expect("Failed to read test file")
}

fn split_and_show<T>(sizer: T, max_size: usize)
where
    T: Sizer,
{
    let code = read_test_file();
    let lang = tree_sitter_go::language();

    let splitter = Splitter::new(lang, sizer)
        .expect("Failed to create golang splitter")
        .with_max_size(max_size);
    let chunks = splitter.split(&code).expect("Failed to split golang code");

    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}

#[test]
fn split_by_chars() {
    split_and_show(CharCounter, 200);
}

#[cfg(feature = "tokenizers")]
#[test]
fn split_by_tokens_huggingface() {
    use tokenizers::tokenizer::Tokenizer;
    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)
        .expect("Failed to load HuggingFace tokenizer");
    split_and_show(tokenizer, 100);
}

#[cfg(feature = "tiktoken-rs")]
#[test]
fn split_by_tokens_tiktoken() {
    use tiktoken_rs::cl100k_base;
    let bpe = cl100k_base().expect("Failed to load tiktoken tokenizer");
    split_and_show(bpe, 100);
}

#[test]
fn split_by_words() {
    split_and_show(WordCounter, 50);
}
