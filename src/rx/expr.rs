/// Expressions for RXML
#[derive(Debug)]
pub enum Expr {
    TagExpr,
    TagStart,
    TagEnd,
    TagContained,
    // Attributes
    AttrExpr,
    RustExpr,
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

#[derive(Debug, Expr!)]
pub struct TagExpr {}
