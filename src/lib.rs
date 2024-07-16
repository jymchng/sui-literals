mod error;
mod hex;

use hex::transform_stream_hash;
use proc_macro::TokenStream;

#[proc_macro]
pub fn sui_literal(input: TokenStream) -> TokenStream {
    match transform_stream_hash(input) {
        Err(err) => err.into_compiler_error().into(),
        Ok(ts) => ts,
    }
}
