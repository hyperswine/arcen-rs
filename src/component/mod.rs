// In arcen, the default container is a flexbox
// You can use grid and stuff, but that can be annoying, <Flex/> just works

// A box is a static node with default properties

// maybe make them (Node) with specific properties

use crate::renderer::node::Node;

pub struct Box(Node);

pub struct Flex(Node);

pub struct Heading(Node);

pub struct Text(Node);

pub struct Image(Node);
