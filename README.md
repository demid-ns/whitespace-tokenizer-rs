# Whitespace Tokenizer

A simple whitespace tokenizer created for learning Rust - intended rather for demonstration with limited practical use.

## How it works

The tokenizer splits text on whitespace boundaries while preserving certain punctuation patterns:

- Contractions (`It's`, `don't`) stay together
- Hyphenated words (`state-of-the-art`) stay together  
- Decimal numbers (`3.14`) stay together
- Time formats (`12:30`) stay together
- Repeated punctuation (`...`, `!!!`) groups as single tokens

## Usage

```rust
use whitespace_tokenizer::whitespace_tokenize;

let text = "It's state-of-the-art! The price is $3.14...";
let tokens = whitespace_tokenize(text);

for token in tokens {
    println!("'{}' [{}..{}]", token.text, token.start, token.end);
}
```

Run the example:
```bash
cargo run --example tokenize
```

## Philosophy

Simple approach focusing on common text patterns rather than comprehensive linguistic rules. Designed for learning purposes and basic text processing scenarios.