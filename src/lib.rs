use crate::debug_masked::debug_masked_derive_impl;
use crate::display_masked::display_masked_derive_impl;
use proc_macro::TokenStream;

mod debug_masked;
mod display_masked;

#[proc_macro_derive(DisplayMasked, attributes(masked))]
pub fn display_masked_derive(input: TokenStream) -> TokenStream {
    display_masked_derive_impl(input)
}

#[proc_macro_derive(DebugMasked, attributes(masked))]
pub fn debug_masked_derive(input: TokenStream) -> TokenStream {
    debug_masked_derive_impl(input)
}
