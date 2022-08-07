/// Bundle your arcen code (rust crate) into a native Neutron Arc App
pub mod bundle;
pub mod component;
/// Parser for ArcXML (AXML), a yew functional like syntax for describing a webpage
pub mod parser;
/// Useful components and functions often used, and per os
pub mod prelude;
pub mod renderer;
/// Lightweight type to pass around and into wgpu
pub mod types;
pub use prelude::*;

pub fn start_arcen() {}
