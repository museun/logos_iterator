# logos_iterator

This is an simple iterator over the [`Logos`](https://docs.rs/logos/) crate.

**NOTE**: Currently it uses the 0.10 pre-release. 

### Example
```rust
// see the documentation for Logos on how this derive works
#[derive(Logos, PartialEq, Clone, Copy, Debug)]
enum Token {
    #[end]
    Eof,
    #[error]
    Unknown,
    #[regex = "[0-9]"]
    Digit,
    #[token = "+"]
    Plus,
    #[token = "-"]
    Minus,
    #[token = "="]
    Equal,
    #[token = ";"]
    End,
    #[regex = "\r?\n"]
    NewLine,
}

// create the iterator from the input source with the token type
// this can generally elide the 'input source', so Lexer::<YourToken, _>
let tokens = Lexer::<Token, &str>::new("1 + 1 = 2;\n2 + 2 = 4;")
    .map(|k| k.item) // remove span information.
                     // each item will be a WithSpan { item, span }
                     // where the span is the byte location (start..end)
                     // of the token in the input source.
    .collect::<Vec<_>>();

assert_eq!(
    tokens,
    vec![
        Token::Digit,
        Token::Plus,
        Token::Digit,
        Token::Equal,
        Token::Digit,
        Token::End,
        Token::NewLine,
        Token::Digit,
        Token::Plus,
        Token::Digit,
        Token::Equal,
        Token::Digit,
        Token::End,
    ]
);
```
