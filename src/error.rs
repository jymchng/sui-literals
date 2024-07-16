use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::result::Result;
use thiserror::Error;

pub(crate) type ParsingResult<T> = Result<T, ParseTokenStreamError>;
pub(crate) type GenerationTokenResult<T> = Result<T, GenerateTokenStreamError>;
pub(crate) type TransformationTokenResult<T> = Result<T, TransformTokenStreamError>;

#[derive(Error, Debug)]
pub enum TransformTokenStreamError {
    #[error("Failed to transform token stream: {0}")]
    TransformError(String, Span),
}

impl From<ParseTokenStreamError> for TransformTokenStreamError {
    fn from(error: ParseTokenStreamError) -> Self {
        match error {
            ParseTokenStreamError::ParseError(msg, span) => {
                TransformTokenStreamError::TransformError(msg, span)
            }
        }
    }
}

impl From<GenerateTokenStreamError> for TransformTokenStreamError {
    fn from(error: GenerateTokenStreamError) -> Self {
        match error {
            GenerateTokenStreamError::GenerationError(msg, span) => {
                TransformTokenStreamError::TransformError(msg, span)
            }
        }
    }
}

impl TransformTokenStreamError {
    /// Converts the `TransformTokenStreamError` into a compiler error message.
    /// This function takes a `Span` and returns a `TokenTree` representing the compiler error.
    ///
    /// # Arguments
    ///
    /// * `span` - The span of the source code where the error occurred.
    ///
    /// # Returns
    ///
    /// A `TokenTree` representing the compiler error message.
    pub fn into_compiler_error(self) -> TokenTree {
        match self {
            TransformTokenStreamError::TransformError(message, span) => error(span, &message),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseTokenStreamError {
    #[error("Failed to parse token stream: {0}")]
    ParseError(String, Span),
}

/// Represents an error that occurs during the generation of a token stream.
/// This error is used to indicate that the generation process has failed and provides
/// a detailed error message describing the reason for the failure.
#[derive(Error, Debug)]
pub enum GenerateTokenStreamError {
    /// Indicates that the token stream generation has failed.
    /// The associated string provides a detailed error message describing the reason for the failure.
    #[error("Failed to generate token stream: {0}")]
    GenerationError(String, Span),
}

impl ParseTokenStreamError {
    /// Converts the `ParseTokenStreamError` into a compiler error message.
    /// This function takes a `Span` and returns a `TokenTree` representing the compiler error.
    ///
    /// # Arguments
    ///
    /// * `span` - The span of the source code where the error occurred.
    ///
    /// # Returns
    ///
    /// A `TokenTree` representing the compiler error message.
    #[allow(dead_code)]
    pub fn into_compiler_error(self) -> TokenTree {
        match self {
            ParseTokenStreamError::ParseError(message, span) => error(span, &message),
        }
    }
}

impl GenerateTokenStreamError {
    /// Converts the `GenerateTokenStreamError` into a compiler error message.
    /// This function takes a `Span` and returns a `TokenTree` representing the compiler error.
    ///
    /// # Arguments
    ///
    /// * `span` - The span of the source code where the error occurred.
    ///
    /// # Returns
    ///
    /// A `TokenTree` representing the compiler error message.
    #[allow(dead_code)]
    pub fn into_compiler_error(self) -> TokenTree {
        match self {
            GenerateTokenStreamError::GenerationError(message, span) => error(span, &message),
        }
    }
}

/// Constructs a compiler error message.
/// This function takes a `Span` and a message string, and returns a `TokenTree` representing
/// the compiler error message. The error message is constructed using the `compile_error!` macro.
///
/// # Arguments
///
/// * `span` - The span of the source code where the error occurred.
/// * `message` - The error message to be displayed by the compiler.
///
/// # Returns
///
/// A `TokenTree` representing the compiler error message.
fn error(span: Span, message: &str) -> TokenTree {
    // See: https://docs.rs/syn/1.0.70/src/syn/error.rs.html#243
    let tokens = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group({
            let mut group = Group::new(
                Delimiter::Brace,
                TokenStream::from_iter(vec![TokenTree::Literal(Literal::string(message))]),
            );
            group.set_span(span);
            group
        }),
    ]);
    TokenTree::Group(Group::new(Delimiter::None, tokens))
}
