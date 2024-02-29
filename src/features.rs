//! Contains configs that reacts to the chosen Cargo.toml `features`

use proc_macro2::TokenStream;

#[cfg(not(feature="format-display"))]
pub const FORMAT_PLACEHOLDER: &str = "{:?}";
#[cfg(feature="format-display")]
pub const FORMAT_PLACEHOLDER: &str = "{}";

// #[!cfg(feature="structured-logger")]
// const fn log() -> TokenStream {
//     quote! {
//         ::log::log!(#level, #fmt, #items);
//     }
// }
//
// #[cfg(feature="structured-logger")]
// fn log() -> TokenStream {
//     quote!()
// }
