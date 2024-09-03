from typing import Protocol

import pytest
from code_splitter import (
    CharSplitter,
    Chunk,
    HuggingfaceSplitter,
    Language,
    TiktokenSplitter,
    WordSplitter,
)

MARKDOWN_INPUT = b"""This is a **sample** markdown file for testing purposes.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum vestibulum. Cras venenatis euismod malesuada.

## Headers

### Subheader Level 3

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut ac metus id leo consectetur fermentum. Sed nec lobortis dolor.

#### Subheader Level 4

Vestibulum volutpat tellus diam, consequat gravida libero rhoncus ut. Maecenas venenatis nisi ac ligula dapibus tincidunt.

## Lists

### Unordered List

- Item 1
    - Subitem 1.1
    - Subitem 1.2
- Item 2
    - Subitem 2.1
    - Subitem 2.2

### Ordered List

1. First item
2. Second item
    1. Subitem 2.1
    2. Subitem 2.2

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam.

## Links and Images

Here is a [link](https://example.com).

![Logo](https://example.com/logo.png)

Curabitur blandit tempus porttitor. Integer posuere erat a ante venenatis dapibus posuere velit aliquet. Nullam quis risus eget urna mollis ornare vel eu leo.

## Code Blocks

### Inline Code

This is an example of `inline code`.

### Block Code

```rust
fn main() {
    println!("Hello, world!");
}
```
"""


class Splitter(Protocol):
    def split(self, code: bytes) -> list[Chunk]: ...


@pytest.mark.parametrize(
    ("splitter", "max_size"),
    [
        (CharSplitter(language=Language.Markdown, max_size=512), 512),
        (WordSplitter(language=Language.Markdown, max_size=64), 64),
        (TiktokenSplitter(language=Language.Markdown, max_size=128), 128),
        (
            HuggingfaceSplitter(
                language=Language.Markdown,
                max_size=128,
                pretrained_model_name_or_path="bert-base-uncased",
            ),
            128,
        ),
    ],
)
def test_split_markdown(splitter: Splitter, max_size: int) -> None:
    # Split the markdown input text into chunks
    chunks = splitter.split(MARKDOWN_INPUT)

    # Check the size of the chunks is less than the max size
    for chunk in chunks:
        assert chunk.size <= max_size
