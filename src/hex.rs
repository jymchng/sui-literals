#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use crate::error::{
    GenerateTokenStreamError, GenerationTokenResult, ParseTokenStreamError, ParsingResult,
    TransformTokenStreamError, TransformationTokenResult,
};
use proc_macro::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use std::{fmt::Write, str::FromStr};
use sui_types::base_types::SUI_ADDRESS_LENGTH;

const UNDERSCORE: char = '_';

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

fn compute_str_limbs(limbs: &[u8]) -> String {
    let mut limbs_str = String::new();
    let mut limbs_vec = vec![0; SUI_ADDRESS_LENGTH];
    for (limb, b) in limbs_vec.iter_mut().zip(limbs) {
        *limb = *b;
    }
    for limb in limbs_vec {
        let _ = write!(&mut limbs_str, "{limb}_u8, ")
            .map_err(|err| format!("attempt to write `\"{limb}_u8\"` but failed; error: {err}"));
    }
    limbs_str.trim_end_matches(", ").into()
}

/// Construct an `ObjectID` literal from `limbs`.
fn construct_objectid(limbs: &[u8], span: Span) -> GenerationTokenResult<TokenStream> {
    let limbs_str = compute_str_limbs(limbs);
    let source = format!("::sui_types::base_types::ObjectID::new([{limbs_str}])");

    TokenStream::from_str(&source).map_err(|err| {
    GenerateTokenStreamError::GenerationError(
        format!("attempt to generate `TokenStream` from `source` = {source} has failed due to error: {err}"), span)
    })
}

/// Construct a `SuiAddress` literal from `limbs`.
fn construct_address(limbs: &[u8], span: Span) -> GenerationTokenResult<TokenStream> {
    let limbs_str = compute_str_limbs(limbs);
    let object_id_source = format!("::sui_types::base_types::ObjectID::new([{limbs_str}])");
    let source = format!("<::sui_types::base_types::SuiAddress::from({object_id_source})>");

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
    let cloned_source = source;
    let (value, suffix) = cloned_source.split_at(suffix_index);
    let value = value.strip_suffix(UNDERSCORE).unwrap_or(value);
    let suffix = suffix.strip_prefix(UNDERSCORE).unwrap_or(value);
    let (_, which) = suffix.split_at(1);
    let address_or_object = TransformInto::from_str(which, span)?;

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
