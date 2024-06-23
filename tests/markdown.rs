use std::fs;

use code_splitter::{Splitter, WordCounter};

#[test]
fn with_word_counter() {
    let code = fs::read("tests/testdata/example.md").unwrap();
    let lang = tree_sitter_md::language();

    let splitter = Splitter::new(lang, WordCounter).unwrap().with_max_size(50);
    let chunks = splitter.split(&code).unwrap();
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{i} {chunk}\n");
    }
}
