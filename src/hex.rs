#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{fmt::Write, str::FromStr};
use crate::error::{GenerationTokenResult, ParsingResult, ParseTokenStreamError, GenerateTokenStreamError};

/// Construct a `H{bits}` literal from `limbs`.
fn construct(bits: usize, limbs: &[u8]) -> GenerationTokenResult<TokenStream> {
    let mut limbs_str = String::new();
    let mut limbs_vec = vec![0; bits / 8];
    for (limb, b) in limbs_vec.iter_mut().zip(limbs) {
        *limb = *b;
    }
    for limb in limbs_vec {
        write!(&mut limbs_str, "{limb}_u8, ").unwrap();
    }
    let limbs_str = limbs_str.trim_end_matches(", ");

    let source = format!("::ethers::core::types::H{bits}([{limbs_str}])");

    Ok(TokenStream::from_str(&source).unwrap())
}

fn parse_suffix(source: &Literal) -> GenerationTokenResult<(usize, &str)> {
    // Parse into value, bits, and base type.]
    let span = source.span();
    let source = source.to_string();
    let suffix_index = source.rfind('H')?;
    let (value, suffix) = source.split_at(suffix_index);
    let value = value.strip_suffix('_').unwrap_or(value);
    let (_, bits) = suffix.split_at(1);
    let bits = bits.parse::<usize>().map_err(|e| {
        ParseTokenStreamError::ParseError(format!(
            "unable to parse {source} to `bits` and convert it into type `usize`"
        ))
    })?;

    Some((bits, value))
}

/// Transforms a [`Literal`] and returns the substitute [`TokenStream`].
fn transform_literal(source: &Literal) -> Result<Option<TokenStream>, String> {
    // Check if literal has a suffix we accept
    let Some((bits, value)) = parse_suffix(source) else {
        return Ok(None);
    };

    let value = value.strip_prefix("0x").unwrap_or(value);

    // Parse `value` into limbs.
    // At this point we are confident the literal was for us, so we throw errors.
    let limbs = hex::decode(value).map_err(|e| format!("hex error: {e}"))?;

    Ok(Some(construct(bits, &limbs)))
}

/// Recurse down tree and transform all literals.
fn transform_tree(tree: TokenTree) -> TokenTree {
    match tree {
        TokenTree::Group(group) => {
            let delimiter = group.delimiter();
            let span = group.span();
            let stream = transform_stream_hash(group.stream());
            let mut transformed = Group::new(delimiter, stream);
            transformed.set_span(span);
            TokenTree::Group(transformed)
        }
        TokenTree::Literal(a) => {
            let span = a.span();
            let source = a.to_string();
            let mut tree = match transform_literal(&a) {
                Ok(Some(stream)) => TokenTree::Group({
                    let mut group = Group::new(Delimiter::None, stream);
                    group.set_span(span);
                    group
                }),
                Ok(None) => TokenTree::Literal(a),
                Err(message) => error(span, &message),
            };
            tree.set_span(span);
            tree
        }
        tree => tree,
    }
}

/// Iterate over a [`TokenStream`] and transform all [`TokenTree`]s.
pub fn transform_stream_hash(stream: TokenStream) -> TokenStream {
    stream.into_iter().map(transform_tree).collect()
}