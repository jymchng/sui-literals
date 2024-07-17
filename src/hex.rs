//! This example demonstrates a compile-time failure when using the `sui_literal` macro with an invalid suffix.
//! The suffix `_obct` is not recognized and will cause a compilation error.
//!
//! ```compile_fail
//! use sui_literals::sui_literal;
//! use sui_types::base_types::{ObjectID, SuiAddress};
//! use std::str::FromStr;
//!
//! let object_id = sui_literal!(0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0_obct);
//! ```
//!
//! This example demonstrates a compile-time failure when using the `sui_literal` macro with an invalid suffix.
//! The suffix `_obct` is not recognized and will cause a compilation error.
//!
//! ```compile_fail
//! use sui_literals::sui_literal;
//! use sui_types::base_types::{ObjectID, SuiAddress};
//! use std::str::FromStr;
//!
//! let sui_address = sui_literal!(0x01b0d52321ce82d032430f859c6df081d56d89445db2d624f0_obct);
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use crate::error::{
    GenerateTokenStreamError, GenerationTokenResult, ParseTokenStreamError, ParsingResult,
    TransformTokenStreamError, TransformationTokenResult,
};
use debug_print::debug_eprintln;
use proc_macro::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use std::{fmt::Write, str::FromStr};

const UNDERSCORE: char = '_';
const SUI_ADDRESS_BYTE_LENGTH: usize = 32;

enum TransformInto {
    SuiAddress,
    ObjectID,
}

impl TransformInto {
    fn from_str(which: &str, span: Span) -> ParsingResult<Self> {
        match which {
            "address" => {
                Ok(Self::SuiAddress)
            },
            "object" => {
                Ok(Self::ObjectID)
            },
            _ => {
                Err(ParseTokenStreamError::ParseError(format!(
                    "the suffix following the literal must be either `address` or `object`, it is {which}"
                ), span))
            }
        }
    }
}

fn compute_str_limbs(limbs: &[u8], span: Span) -> GenerationTokenResult<String> {
    debug_eprintln!("inside `compute_str_limbs`; limbs = {:?}", &limbs);
    if limbs.len() > 32 {
        return Err(GenerateTokenStreamError::GenerationError(
            format!(
                "error: the number of limbs is not `{SUI_ADDRESS_BYTE_LENGTH}`, but it is `{}`",
                limbs.len()
            ),
            span,
        ));
    }
    let mut limbs_str = String::new();
    let mut limbs_vec = vec![0; SUI_ADDRESS_BYTE_LENGTH];
    for (limb, b) in limbs_vec.iter_mut().zip(limbs) {
        *limb = *b;
    }
    for limb in limbs_vec {
        let _ = write!(&mut limbs_str, "{limb}_u8, ")
            .map_err(|err| format!("attempt to write `\"{limb}_u8\"` but failed; error: {err}"));
    }
    let result: String = limbs_str.trim_end_matches(", ").into();
    debug_eprintln!("inside `compute_str_limbs`; result = {:?}", &result);
    Ok(result)
}

/// Construct an `ObjectID` literal from `limbs`.
fn construct_objectid(limbs: &[u8], span: Span) -> GenerationTokenResult<TokenStream> {
    let limbs_str = compute_str_limbs(limbs, span)?;
    let source = format!(
        "{{
        use ::sui_types as __suitypes;
        __suitypes::base_types::ObjectID::new([{limbs_str}])
    }}"
    );

    TokenStream::from_str(&source).map_err(|err| {
    GenerateTokenStreamError::GenerationError(
        format!("attempt to generate `TokenStream` from `source` = {source} has failed due to error: {err}"), span)
    })
}

/// Construct a `SuiAddress` literal from `limbs`.
fn construct_address(limbs: &[u8], span: Span) -> GenerationTokenResult<TokenStream> {
    let limbs_str = compute_str_limbs(limbs, span)?;
    let object_id_source = format!("__suitypes::base_types::ObjectID::new([{limbs_str}])");
    let source = format!(
        "{{
        use ::sui_types as __suitypes;
        __suitypes::base_types::SuiAddress::from({object_id_source})
    }}"
    );
    debug_eprintln!("inside `construct_address` function; `source` = {source}");
    TokenStream::from_str(&source).map_err(|err| {
        GenerateTokenStreamError::GenerationError(
            format!("attempt to generate `TokenStream` from `source` = {source} has failed due to error: {err}"), span)
    })
}

fn parse_suffix(source: &Literal) -> ParsingResult<(TransformInto, String)> {
    let span = source.span();
    let source = source.to_string();
    let suffix_index = source.rfind(UNDERSCORE).ok_or_else(|| {
        ParseTokenStreamError::ParseError(format!(
            "unable to find the delimiter `{UNDERSCORE}`; you must indicate whether you want to parse the literal as a `SuiAddress` by suffixing it with `'_address'` or as an `ObjectId` by suffixing it with `'_object'`"
        ), span)
    })?;
    debug_eprintln!("inside `parse_suffix` function; `suffix_index` = {suffix_index}");
    let cloned_source = source;
    let (value, suffix) = cloned_source.split_at(suffix_index);
    let value = value.strip_suffix(UNDERSCORE).unwrap_or(value);
    let suffix = suffix.strip_prefix(UNDERSCORE).unwrap_or(value);
    debug_eprintln!("inside `parse_suffix` function; `value` = {value}");
    debug_eprintln!("inside `parse_suffix` function; `suffix` = {suffix}");
    let address_or_object = TransformInto::from_str(suffix, span)?;

    Ok((address_or_object, value.into()))
}

/// Transforms a [`Literal`] and returns the substitute [`TokenStream`].
fn transform_literal(source: &Literal) -> TransformationTokenResult<TokenStream> {
    let (address_or_object, value) = parse_suffix(source)?;

    let value = value.strip_prefix("0x").unwrap_or(&value);

    let limbs = hex::decode(value).map_err(|e| {
        ParseTokenStreamError::ParseError(
            format!("unable to decode `{value}` into hexadecimal; error: {e}"),
            source.span(),
        )
    })?;

    match address_or_object {
        TransformInto::ObjectID => Ok(construct_objectid(&limbs, source.span())?),
        TransformInto::SuiAddress => Ok(construct_address(&limbs, source.span())?),
    }
}

/// Recurse down tree and transform all literals.
fn transform_tree(tree: TokenTree) -> TransformationTokenResult<TokenTree> {
    match tree {
        TokenTree::Group(group) => {
            let delimiter = group.delimiter();
            let span = group.span();
            let stream = transform_stream_hash(group.stream())?;
            let mut transformed = Group::new(delimiter, stream);
            transformed.set_span(span);
            Ok(TokenTree::Group(transformed))
        }
        TokenTree::Literal(a) => {
            let span = a.span();
            let _source = a.to_string();
            let mut tree = match transform_literal(&a) {
                Ok(stream) => TokenTree::Group({
                    let mut group = Group::new(Delimiter::None, stream);
                    group.set_span(span);
                    group
                }),
                Err(message) => {
                    return Err(message);}
            };
            tree.set_span(span);
            Ok(tree)
        }
        tree => {
            Err(TransformTokenStreamError::TransformError(
                "error: only `TokenTree::Group` and `TokenTree::Literal` are allowed in the `TokenStream`".to_string(),
                tree.span()))
        },
    }
}

/// Iterate over a [`TokenStream`] and transform all [`TokenTree`]s.
pub fn transform_stream_hash(stream: TokenStream) -> TransformationTokenResult<TokenStream> {
    let mut result = TokenStream::new();
    for tree in stream {
        result.extend(TokenStream::from(transform_tree(tree)?));
    }
    Ok(result)
}
