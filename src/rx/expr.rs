/// Expressions for RXML
#[derive(Debug)]
pub enum Expr {
    ElementDef(Box<ElementDef>),
    Literal(Literal),
    ScopeExpr(Box<ScopeExpr>),
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
    pub attrs: Vec<Attribute>,
    pub children: Vec<ElementDef>,
}

/// An attribute is a named parameter (instead of positional) and describes a property of the current node, and any child nodes that wish to inherit it
/// Most normal attributes like position and stuff are implicitly passed to child elements
/// Custom attributes can be passed down as 'properties' to child elements' attributes if the child element's attributes explicitly parametrise themselves with an attribute specified by their direct parent
#[derive(Debug)]
pub struct Attribute {}

// Maybe also add a way to bring up properties from child -> parent. And from grandparent -> child. Flutter uses keys, which ehh
// Maybe also redux

#[derive(Debug)]
pub enum Literal {
    Numeric(f32),
    Array(Box<Literal>),
    String(String),
}

impl From<Literal> for Expr {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Numeric(n) => Expr::Literal(Literal::Numeric(n)),
            Literal::Array(a) => Expr::Literal(Literal::Array(a)),
            Literal::String(s) => Expr::Literal(Literal::String(s)),
        }
    }
}

// If you "detect" a rust expr, just leave it?
// and keep searching for an rx expr?

// RUST EXPR. Basically checks if a valid rx rust expression exists and returns that, otherwise, it will not match anything and raise an error
#[macro_export]
macro_rules! rust_expr {
    ($($e:expr)*) => {
        $($e)*
    };
}

#[derive(Debug, Expr!)]
pub struct ScopeExpr {
    pub expr: Expr,
    pub string: String,
}

impl ToString for ScopeExpr {
    /// Hmm maybe add a string: to ScopeExpr which is added in parsing
    fn to_string(&self) -> String {
        self.string.clone()
    }
}
