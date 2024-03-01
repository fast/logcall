//! Contains configs that reacts to the chosen Cargo.toml `features`

use proc_macro2::TokenStream;

#[cfg(not(feature="format-display"))]
pub const FORMAT_PLACEHOLDER: &str = "{:?}";
#[cfg(feature="format-display")]
pub const FORMAT_PLACEHOLDER: &str = "{}";

// #[cfg(feature="structured-logging")]
// pub const LOG_TRAILING_SEPARATOR: char = ';';
// #[cfg(not(feature="structured-logging"))]
// pub const LOG_TRAILING_SEPARATOR: char = ',';
//
