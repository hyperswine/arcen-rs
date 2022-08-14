// In arcen, the default container is a flexbox
// You can use grid and stuff, but that can be annoying, <Flex/> just works

// A box is a static node with default properties

// maybe make them (Node) with specific properties

use crate::renderer::node::Node;

// Maybe make a derive(Node) instead, with an into_node() method

pub struct Box(Node);

pub struct Flex(Node);

pub struct Heading(Node);

/// Mostly Sugar, becomes incorporated as attr.text instead of a 'child' node
pub struct Text(String);

impl Text {
    // Change the text
    pub fn text(&mut self) {}
}

/// Mostly Sugar, becomes incorporated as attr.image instead of a 'child' node
pub struct Image(String);

pub struct Component;
