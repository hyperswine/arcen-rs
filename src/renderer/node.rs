use crate::types::{Vec3b, Vec4fh};

pub type Padding = Vec4fh;
pub type Borders = Vec4fh;
pub type ColorRGB = Vec3b;

// rgb
// u8,u8,u8
// text: Text
// Text (font: Font). So we use that for children as well
// Background color is not inherited, but color is
// If image is specified, then tries to render the image in the container, over the background color and text

pub type Font = String;
/// An image is simply a wgpu Texture if using wgpu backend
pub type Image = String;
pub type Surface3D = String;

#[repr(C)]
#[derive(Debug)]
pub struct Text {
    text: String,
    font: Font,
}

#[repr(C)]
#[derive(Debug)]
pub struct Node {
    padding: Padding,
    borders: Borders,
    background_color: ColorRGB,
    color: ColorRGB,
    text: Text,
    image: Image,

    children: Vec<Node>,
}

// Each node has a certain set of characteristics. Like the CSS box model, everything in arcen is a container. And the container may container subcontainers
// A container is a 2D rectangle consisting of:
// padding, borders, background color, child color, id, etc.
// Certain features like fonts, images, colors are extras that are contained in a container. Instead of being a container itself, they are "attributes" like padding. There is also a 3D view, which also an attr
