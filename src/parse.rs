// std
use std::fmt;
use std::unimplemented;

// crates

// local
use crate::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Import(String),
    Variable(Variable),
    Pattern(Pattern),
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
                    unimplemented!()
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String { value: String },
    Number { value: f64 },
    Boolean { value: bool },
    Custom { value: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub parameters: Vec<String>,
    pub inner: String,
    pub rtrn: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Constant {
        name: String,
        value: String,
    },
    Mutable {
        name: String,
        value: String,
    },
    Man {
        display: String,
        name: String,
        default: String,
    },
}

pub trait Output {
    fn to_output(&self) -> Result<String, Error>;
}
