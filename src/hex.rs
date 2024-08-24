//! # `sui-literals` Macro Library
//!
//! This library provides macros and utilities for transforming literals into `ObjectID` or `SuiAddress`
//! types from hexadecimal representations.
//!
//! ## Errors
//!
//! Errors are handled through custom error types:
//!
//! - `GenerateTokenStreamError`: Indicates errors during token stream generation.
//! - `ParseTokenStreamError`: Indicates errors during token stream parsing.
//! - `TransformTokenStreamError`: Indicates errors during token stream transformation.
//!
//! ## Constants
//!
//! - `UNDERSCORE`: Constant character `_` used for suffix parsing.
//! - `SUI_ADDRESS_BYTE_LENGTH`: Length of bytes for `SuiAddress`.
//!
//! ## Enum `TransformInto`
//!
//! Enumerates the transformation target types:
//!
//! - `SuiAddress`: Indicates transformation into `SuiAddress`.
//! - `ObjectID`: Indicates transformation into `ObjectID`.
//!
//! ## Function `TransformInto::from_str`
//!
//! Parses a string slice to determine the transformation target.
//!
//! ## Function `compute_str_limbs`
//!
//! Computes a string representation of limbs for hexadecimal literals.
//!
//! ## Function `construct_objectid`
//!
//! Constructs an `ObjectID` literal from limbs.
//!
//! ## Function `construct_address`
//!
//! Constructs a `SuiAddress` literal from limbs.
//!
//! ## Function `parse_suffix`
//!
//! Parses the suffix following a literal to determine transformation type and value.
//!
//! ## Function `transform_literal`
//!
//! Transforms a literal into a token stream based on its suffix.
//!
//! ## Function `transform_tree`
//!
//! Recursively transforms all literals within a token tree.
//!
//! ## Function `transform_stream_hash`
//!
//! Iterates over a token stream and transforms all literals within it.
//!
//! # Examples
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
//! let sui_address = sui_literal!(0x01b0d52321ce82d032430f859c6df081d56d89445db2d624f0_object);
//! ```
//!
//! The above example also demonstrates a compile-time failure with an invalid suffix `_obct`.
//!
//! # Notes
//!
//! - Ensure proper suffix (`_address` or `_object`) is used to avoid compilation errors.
//! - Functions handle hexadecimal decoding and token stream generation internally.
//! - Debug prints are enabled to aid in development and troubleshooting.
//!
//! # Usage
//!
//! Integrate the `sui_literals` macros into your Rust projects to efficiently convert hexadecimal literals
//! into `ObjectID` or `SuiAddress` types, ensuring type safety and compile-time checks for transformation suffixes.
//!
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use crate::error::{
    GenerateTokenStreamError, GenerationTokenResult, ParseTokenStreamError, ParsingResult,
    TransformTokenStreamError, TransformationTokenResult,
};
use debug_print::debug_eprintln;
use proc_macro::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use std::{fmt::Write, str::FromStr};

const UNDERSCORE: char = '_';
const O_X: &str = "0x";
const SUI_ADDRESS_BYTE_LENGTH: usize = 32;

/// Enumerates the target types for transformation.
#[derive(Debug)]
enum TransformInto {
    SuiAddress,
    ObjectID,
}

impl TransformInto {
    /// Parses a string slice to determine the transformation target.
    fn from_str(which: &str, span: Span) -> ParsingResult<Self> {
        match which {
            "address" => Ok(Self::SuiAddress),
            "object" => Ok(Self::ObjectID),
            _ => Err(ParseTokenStreamError::ParseError(
                format!("Suffix must be either `address` or `object`, but found `{which}`",),
                span,
            )),
        }
    }
}

/// Computes a string representation of limbs for hexadecimal literals.
fn compute_str_limbs(limbs: &[u8], span: Span) -> GenerationTokenResult<String> {
    debug_eprintln!("inside `compute_str_limbs`; limbs = {:?}", &limbs);

    if limbs.len() > SUI_ADDRESS_BYTE_LENGTH {
        return Err(GenerateTokenStreamError::GenerationError(
            format!(
                "Expected {} limbs, found {}",
                SUI_ADDRESS_BYTE_LENGTH,
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
            .map_err(|err| format!("Failed to write `{limb}_u8`; error: {err}"));
    }

    let result: String = limbs_str.trim_end_matches(", ").into();
    debug_eprintln!("inside `compute_str_limbs`; result = {:?}", &result);
    Ok(result)
}

/// Constructs an `ObjectID` literal from limbs.
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
            format!("Failed to generate `TokenStream` from `source` = {source}; error: {err}"),
            span,
        )
    })
}

/// Constructs a `SuiAddress` literal from limbs.
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
            format!("Failed to generate `TokenStream` from `source` = {source}; error: {err}"),
            span,
        )
    })
}

/// Parses the suffix following a literal to determine transformation type and value.
fn parse_suffix(source: &Literal) -> ParsingResult<(TransformInto, String)> {
    let span = source.span();
    let source = source.to_string();

    let suffix_index = source.rfind(UNDERSCORE).ok_or_else(|| {
        ParseTokenStreamError::ParseError(format!("Unable to find `{UNDERSCORE}` delimiter"), span)
    })?;

    debug_eprintln!("inside `parse_suffix`; `suffix_index` = {suffix_index}");

    let cloned_source = source;
    let (value, suffix) = cloned_source.split_at(suffix_index);
    let value = value.strip_suffix(UNDERSCORE).unwrap_or(value);
    let suffix = suffix.strip_prefix(UNDERSCORE).unwrap_or(value);
    let value = value.strip_prefix(O_X).unwrap_or(value);

    debug_eprintln!("inside `parse_suffix`; `value` = {value}");
    debug_eprintln!("inside `parse_suffix`; `suffix` = {suffix}");

    if value.len() != 64 {
        return Err(ParseTokenStreamError::ParseError(
            "the address cannot be converted into a byte array of size 32".to_string(),
            span,
        ));
    }

    let address_or_object = TransformInto::from_str(suffix, span)?;

    Ok((address_or_object, value.into()))
}

/// Transforms a literal into a token stream based on its suffix.
fn transform_literal(source: &Literal) -> TransformationTokenResult<TokenStream> {
    let (address_or_object, value) = parse_suffix(source)?;

    debug_eprintln!("inside `transform_literal`; `value` = {value}");
    let limbs = hex::decode(&value).map_err(|e| {
        ParseTokenStreamError::ParseError(
            format!("Unable to decode `{}` into hexadecimal; error: {e}", &value),
            source.span(),
        )
    })?;

    match address_or_object {
        TransformInto::ObjectID => Ok(construct_objectid(&limbs, source.span())?),
        TransformInto::SuiAddress => Ok(construct_address(&limbs, source.span())?),
    }
}

/// Iteratively transforms all literals within a token tree.
fn transform_tree(tree: TokenTree) -> TransformationTokenResult<TokenTree> {
    let mut stack = vec![tree];
    let mut result_stack = Vec::new();

    while let Some(current_tree) = stack.pop() {
        match current_tree {
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();
                let span = group.span();
                let transformed_stream = transform_stream_hash(group.stream())?;
                let mut transformed_group = Group::new(delimiter, transformed_stream);
                transformed_group.set_span(span);
                result_stack.push(TokenTree::Group(transformed_group));
            }
            TokenTree::Literal(literal) => {
                let span = literal.span();
                let transformed_tree = match transform_literal(&literal) {
                    Ok(stream) => {
                        let mut group = Group::new(Delimiter::None, stream);
                        group.set_span(span);
                        TokenTree::Group(group)
                    }
                    Err(message) => {
                        return Err(message);
                    }
                };
                transformed_tree.set_span(span);
                result_stack.push(transformed_tree);
            }
            other => {
                return Err(TransformTokenStreamError::TransformError(
                    "Only `TokenTree::Group` and `TokenTree::Literal` are allowed in the `TokenStream`"
                        .to_string(),
                    other.span(),
                ));
            }
        }
    }

    Ok(result_stack.pop().unwrap())
}

/// Iterates over a `TokenStream` and transforms all `TokenTree`s.
pub fn transform_stream_hash(stream: TokenStream) -> TransformationTokenResult<TokenStream> {
    let mut result = TokenStream::new();
    let mut stack: Vec<TokenTree> = stream.into_iter().collect();

    while let Some(tree) = stack.pop() {
        result.extend(TokenStream::from(transform_tree(tree)?));
    }

    Ok(result)
}

