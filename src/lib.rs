/// Bundle your arcen code into a native Neutron Arc App
pub mod bundle;
/// Useful components and functions often used, and per os
pub mod prelude;
/// Rx declarative syntax
pub mod rx;
/// Lightweight type to pass around and into wgpu
pub use prelude::*;

#[macro_use]
extern crate macro_rules_attribute;
