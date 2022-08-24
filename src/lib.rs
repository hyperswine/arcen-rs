/// Bundle your arcen code (rust crate) into a native Neutron Arc App
pub mod bundle;
pub mod component;
/// Useful components and functions often used, and per os
pub mod prelude;
pub mod renderer;
/// Rx declarative syntax
pub mod rx;
/// Lightweight type to pass around and into wgpu
pub mod types;
pub use prelude::*;

// USE
#[macro_use]
extern crate macro_rules_attribute;

pub fn start_arcen() {}

// #[derive(Debug, StructOpt)]
// #[structopt()]
// pub struct Options {
//     /// Use Vulkan debug layer (requires Vulkan SDK installed)
//     #[structopt(short, long)]
//     debug_layer: bool,
// }
