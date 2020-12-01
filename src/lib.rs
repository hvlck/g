#[macro_use]
// std

// external
use pest::Parser;
use pest_derive::*;

// local

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct GParser;

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
