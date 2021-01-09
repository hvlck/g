// std
use std::fmt;
use std::unimplemented;

// crates

// local
use crate::Error;

/// Represents a node in the Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// Import statement (`use "x"`; corresponds to `import("x")` in JavaScript)
    Import(String),
    /// Variable declaration
    Variable(Variable),
    /// Pattern/function
    Pattern(Pattern),
    /// Error in parsing
    Error(Error),
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Import(string) => {
                if string.starts_with("http") == false {
                    let mut s = String::from("./");
                    s.push_str(string.as_str());
                    write!(f, "use \"{}\"", s)
                } else {
                    write!(f, "{}", string)
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Output for AstNode {
    fn to_output(&self) -> Result<String, Error> {
        match self {
            AstNode::Import(string) => {
                if string.len() == 0 {
                    Err(Error::NoImport)
                } else {
                    let mut import = String::from("import(\"");
                    if string.starts_with("http") == false {
                        let mut s = String::from("./");
                        s.push_str(string.as_str());

                        import.push_str(s.as_str());
                        import.push_str("\");");

                        Ok(import)
                    } else {
                        import.push_str(string.as_str());
                        import.push_str("\");");

                        Ok(import)
                    }
                }
            }
            AstNode::Variable(var) => match var {
                Variable::Constant { name, value } => {
                    unimplemented!()
                }
                Variable::Mutable { name, value } => {
                    let mut out = String::from("let ");

                    out.push_str(name.as_str());
                    out.push_str("=");

                    out.push_str(value);
                    out.push(';');

                    Ok(out)
                }
                Variable::Man {
                    display,
                    name,
                    default,
                } => {
                    let mut out = String::from("\"");
                    out.push_str(name);

                    out.push_str("\":{default:");
                    out.push_str(default.as_str());

                    out.push_str(",display:");
                    out.push_str(display.as_str());
                    out.push_str("}");

                    Ok(out)
                }
            },
            _ => unimplemented!(),
        }
    }
}

/// Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String { value: String },
    Number { value: f64 },
    Boolean { value: bool },
    Custom { value: String },
}

/// Similar to a function in other languages, a pattern is used to draw something repeatedly
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    /// Parameters/arguments
    pub parameters: Vec<String>,
    /// Inner source of the pattern
    pub inner: Vec<AstNode>,
    /// Return value and type of the function
    pub rtrn: (Type, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    /// Immutable variable (`const`)
    Constant {
        /// Internal name of the variable
        name: String,
        /// Value of the variable.
        value: String,
    },
    /// Mutable variable (`let`)
    Mutable {
        /// Internal name of the variable
        name: String,
        /// Value of the variable at initialisation
        value: String,
    },
    /// Variable that can be manipulated by the user at runtime.
    Man {
        /// Display name, shown to user
        display: String,
        /// Internal name of the variable
        name: String,
        /// Default value of the variable
        default: String,
    },
}

/// Implemtors of this trait can generate JavaScript source code from an AST node.
pub trait Output {
    fn to_output(&self) -> Result<String, Error>;
}
