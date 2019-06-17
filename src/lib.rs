//! This is an simple iterator over the [`Logos`](https://docs.rs/logos/) crate.
//!
//! ## Example
//! ```rust
//! # use logos_iterator::Lexer;
//! # use logos::Logos;
//! // see the documentation for Logos on how this derive works
//! #[derive(Logos, PartialEq, Clone, Copy, Debug)]
//! enum Token {
//!     #[end]
//!     Eof,
//!     #[error]
//!     Unknown,
//!     #[regex = "[0-9]"]
//!     Digit,
//!     #[token = "+"]
//!     Plus,
//!     #[token = "-"]
//!     Minus,
//!     #[token = "="]
//!     Equal,
//!     #[token = ";"]
//!     End,
//!     #[regex = "\r?\n"]
//!     NewLine,
//! }
//!
//! // create the iterator from the input source with the token type
//! // this can generally elide the 'input source', so Lexer::<YourToken, _>
//! let tokens = Lexer::<Token, &str>::new("1 + 1 = 2;\n2 + 2 = 4;")
//!     .map(|k| k.item) // remove span information.
//!                      // each item will be a WithSpan { item, span }
//!                      // where the span is the byte location (start..end)
//!                      // of the token in the input source.
//!     .collect::<Vec<_>>();
//!
//! assert_eq!(
//!     tokens,
//!     vec![
//!         Token::Digit,
//!         Token::Plus,
//!         Token::Digit,
//!         Token::Equal,
//!         Token::Digit,
//!         Token::End,
//!         Token::NewLine,
//!         Token::Digit,
//!         Token::Plus,
//!         Token::Digit,
//!         Token::Equal,
//!         Token::Digit,
//!         Token::End,
//!     ]
//! );
//! ```

/// A lexer is an iterator over an input source, `S` that yields token `T` until
/// the
/// [`#[logos::end]`](https://docs.rs/logos/latest/logos/trait.Logos.html#associatedconstant.END)
/// token is found
///
/// The yielded element is wrapped with a [`Span`](./struct.Span.html), which is
/// the byte offset into the source that the token is located at
pub struct Lexer<T, S>(::logos::Lexer<T, S>)
where
    T: PartialEq<T> + ::logos::Logos;

impl<'a, T, S> Lexer<T, S>
where
    T: PartialEq<T> + ::logos::Logos + ::logos::source::WithSource<S>,
    S: ::logos::source::Source<'a>,
{
    /// Create a new lexer from the source `S`
    ///
    /// * T is the Token (that
    ///   [`#[derive(Logos)]`](https://docs.rs/logos/latest/logos/trait.Logos.html)
    ///   applies to)
    /// * S is something that implements
    ///   [`logos::Source`](https://docs.rs/logos/latest/logos/source/trait.Source.html)
    ///   (`&str`, `&[u8]`, etc)
    pub fn new(s: S) -> Self {
        Self(T::lexer(s))
    }
}

impl<'a, T, S> Iterator for Lexer<T, S>
where
    T: Copy + Clone + PartialEq<T>,
    T: ::logos::Logos + ::logos::source::WithSource<S>,
    S: ::logos::source::Source<'a>,
{
    type Item = WithSpan<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.token == T::END {
            return None;
        }

        let token = self.0.token;
        let range = self.0.range();
        let span = Span {
            start: range.start,
            end: range.end,
        };

        self.0.advance();
        Some(WithSpan::new(token, span))
    }
}

/// `WithSpan` wraps something with a [`Span`](./struct.Span.html)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithSpan<T> {
    pub item: T,
    pub span: Span,
}

impl<T> WithSpan<T> {
    /// Wrap `item` with [`span`](./struct.Span.html)
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }
}

/// `Span` represents a `start`..`end` range
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
