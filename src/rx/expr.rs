/// Expressions for RXML
#[derive(Debug)]
pub enum Expr {
    ElementDef(Box<ElementDef>),
}

macro_rules! Expr {
    ($(#[$attr:meta])* $vis:vis struct $name:ident $($stmt:tt)*) => {
        impl From<$name> for Expr {
            fn from(from: $name) -> Expr {
                Expr::$name(Box::new(from))
            }
        }
    };
}

// KEY EXPRESSIONS

/// An element can have a list of attributes and a list of child elements
#[derive(Debug, Expr!)]
pub struct ElementDef {
    attrs: Vec<Attribute>,
    children: Vec<ElementDef>,
}

/// An attribute is a named parameter (instead of positional) and describes a property of the current node, and any child nodes that wish to inherit it
/// Most normal attributes like position and stuff are implicitly passed to child elements
/// Custom attributes can be passed down as 'properties' to child elements' attributes if the child element's attributes explicitly parametrise themselves with an attribute specified by their direct parent
#[derive(Debug)]
pub struct Attribute {}

// Maybe also add a way to bring up properties from child -> parent. And from grandparent -> child. Flutter uses keys, which ehh
// Maybe also redux
