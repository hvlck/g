#[macro_use]
// std

// external
use pest::{error::Error as PestError, Parser};
use pest_derive::*;

// local
pub mod error;
use error::Error;

pub mod parse;
use parse::*;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct GParser;

/// Parses a given source (`src`) string into JavaScript.
fn parse(src: &str) -> Result<String, PestError<Rule>> {
    let mut ast: Vec<AstNode> = Vec::new();

    let exp = GParser::parse(Rule::any, src)?;
    for pair in exp {
        match pair.as_rule() {
            Rule::any => {
                ast.push(parse_to_ast(pair));
            }
            _ => ast.push(AstNode::Error(Error::InvalidInput)),
        }
    }

    let mut final_output = String::new();
    for i in ast {
        final_output.push_str(i.to_output().unwrap().as_str());
    }

    Ok(final_output)
}

fn parse_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::any | Rule::variable => parse_to_ast(pair.into_inner().next().unwrap()),
        Rule::import_pkg => {
            let mut pair = pair.into_inner();
            let import = match pair.next() {
                Some(v) => v.as_str().to_string(),
                None => return AstNode::Error(Error::NoImport),
            };
            let import = import.replace("\"", "");
            AstNode::Import(String::from(import))
        }
        Rule::man_variable => {
            let mut pair = pair.into_inner();
            let display = pair.next().unwrap().as_str().to_string();
            let name = pair.next().unwrap().as_str().to_string();
            let value = pair.next().unwrap().as_str().to_string();
            AstNode::Variable(Variable::Man {
                display,
                name,
                default: value,
            })
        }
        _ => AstNode::Error(Error::InvalidInput),
    }
}

#[cfg(test)]
mod output_tests {
    use super::*;
    #[test]
    fn test_import() {
        let import = parse(r#"use "math""#);
        assert!(import.is_ok());
        assert_eq!(import.unwrap(), r#"import("./math");"#);
    }

    #[test]
    fn test_man_variable() {
        let var = parse(r#"man "Decay Rate" as decay_rate = 0.9"#);
        assert!(var.is_ok());
        assert_eq!(
            var.unwrap(),
            r#""decay_rate":{default:0.9,display:"Decay Rate"}"#
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_operation() {
        let op = GParser::parse(Rule::operation, "+");
    }

    #[test]
    fn test_number() {
        let num = GParser::parse(Rule::number, "1");

        let larger_num = GParser::parse(Rule::number, "9999999999");

        assert!(num.is_ok());
        assert!(larger_num.is_ok());
    }

    #[test]
    fn test_expression() {
        let exp = GParser::parse(Rule::expression, "1+1");
        assert!(exp.is_ok());
    }

    #[test]
    fn test_string() {
        let string = GParser::parse(Rule::string, "\"This is an example string.\"");
        assert!(string.is_ok());
    }

    #[test]
    fn test_valid_variable_names() {
        let test = GParser::parse(Rule::valid_variable_name, "generic_age");
        assert!(test.is_ok());
    }

    #[test]
    fn test_let_variable() {
        let variable = GParser::parse(Rule::let_variable, "let generic_age = 22");
        assert!(variable.is_ok());
    }

    #[test]
    fn test_man_variable() {
        let variable = GParser::parse(Rule::man_variable, "man \"Decay Rate\" as decay_rate = 22");
        assert!(variable.is_ok());
    }

    #[test]
    fn test_const_variable() {
        let variable = GParser::parse(Rule::const_variable, "const generic_age = 22");
        assert!(variable.is_ok());
    }

    #[test]
    fn test_use_statement() {
        let use_with_std = GParser::parse(Rule::import_pkg, "use \"math\"");
        assert!(use_with_std.is_ok());
    }

    #[test]
    fn test_internal_comment() {
        let valid_comment =
            GParser::parse(Rule::internal_comment, "// This is a generic comment. //");
        assert!(valid_comment.is_ok());
    }

    #[test]
    fn test_process_comment() {
        let valid_comment = GParser::parse(Rule::process_comment, "/// Add 2 to 2 ///");
        assert!(valid_comment.is_ok());
    }

    #[test]
    fn test_doc_comment() {
        let valid_comment = GParser::parse(Rule::doc_comment, "//! Here is some documentation. //");
        assert!(valid_comment.is_ok());
    }
}
