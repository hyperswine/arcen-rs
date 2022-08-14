use crate::types::{Vec2fh, Vec3b, Vec4fh};

pub type Padding = Vec4fh;
pub type Borders = Vec4fh;
pub type ColorRGB = Vec3b;

// rgb
// u8,u8,u8
// text: Text
// Text (font: Font). So we use that for children as well
// Background color is not inherited, but color is
// If image is specified, then tries to render the image in the container, over the background color and text

pub type FontFamily = String;
pub type FontSize = usize;
/// An image is simply a wgpu Texture if using wgpu backend
pub type Image = String;
pub type Surface3D = String;

#[repr(C)]
#[derive(Debug)]
pub struct Font {
    // should be enum though string is fine for custom and future fonts
    pub font_family: FontFamily,
    // in pixels or %
    pub font_size: FontSize,
}

// positions are calculated relative to a root position base
// which can be defined as anything you want
// on arcdesktop, it is the top left corner (0,0), similar to html/css

#[repr(C)]
#[derive(Debug)]
pub enum PositionType {
    Relative,
    Absolute,
}

#[repr(C)]
#[derive(Debug)]
pub struct Position {
    position_type: PositionType,
    position: Vec2fh,
}

#[repr(C)]
#[derive(Debug)]
pub enum ContentAlignment {
    Start,
    Center,
    End,
}

#[repr(C)]
#[derive(Debug)]
pub enum ContentSpacing {
    Around,
    Between,
    Even,
}

#[repr(C)]
#[derive(Debug)]
pub enum Axis {
    Row,
    Col,
}

#[repr(C)]
#[derive(Debug)]
pub struct Animate {
    pub duration_seconds: f64,
    pub translate_to: Vec2fh,
    pub rotate_by: Vec2fh,
    pub scale_by: Vec2fh,
}

#[repr(C)]
#[derive(Debug)]
pub struct Node {
    pub padding: Padding,
    pub borders: Borders,
    pub background_color: ColorRGB,
    pub color: ColorRGB,
    pub font: Font,
    /// Primitive value
    pub text: Option<String>,
    /// Primitive value, though could just be done at walkthrough time?
    pub image: Option<Image>,
    pub position: Position,
    // An animation list
    pub animate: Vec<Animate>,

    // Maybe have an Option<> for these
    pub children: Vec<Node>,
    pub children_axis: Axis,
    pub children_alignment: ContentAlignment,
    pub children_spacing: ContentSpacing,
}

impl Node {
    pub fn view(&self) {}
}

// Each node has a certain set of characteristics. Like the CSS box model, everything in arcen is a container. And the container may container subcontainers
// A container is a 2D rectangle consisting of:
// padding, borders, background color, child color, id, etc.
// Certain features like fonts, images, colors are extras that are contained in a container. Instead of being a container itself, they are "attributes" like padding. There is also a 3D view, which also an attr
