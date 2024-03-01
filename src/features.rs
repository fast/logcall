//! Contains configs that reacts to the chosen Cargo.toml `features`

#[cfg(not(feature = "format-display"))]
/// Specifies that the realization of parameters and return values should be done using the `Debug` trait
/// (this can be changed to `Display` by enabling the "format-display" feature of this crate)
pub const FORMAT_PLACEHOLDER: &str = "{:?}";
#[cfg(feature = "format-display")]
/// Specifies that the realization of parameters and return values should be done using the `Display` trait
/// (this can be changed to `Debug` by removing the "format-display" feature of this crate)
pub const FORMAT_PLACEHOLDER: &str = "{}";
