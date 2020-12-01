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
        println!("Operation: {:?}", op);
    }

    #[test]
    fn test_number() {
        let num = GParser::parse(Rule::number, "1");
        println!("Number: {:?}", num);

        let larger_num = GParser::parse(Rule::number, "9999999999");
        println!("Larger number: {:?}", larger_num);

        assert!(num.is_ok());
        assert!(larger_num.is_ok());
    }

    #[test]
    fn test_expression() {
        let exp = GParser::parse(Rule::expression, "1+1");
        println!("Expression: {:?}", exp);
        assert!(exp.is_ok());
    }

    #[test]
    fn test_string() {
        let string = GParser::parse(Rule::string, "\"This is an example string.\"");
        println!("String: {:?}", string);
        assert!(string.is_ok());
    }

    #[test]
    fn test_valid_variable_names() {
        let test = GParser::parse(Rule::valid_variable_name, "generic_age");
        println!("Valid variable name: {:?}", test);
        assert!(test.is_ok());
    }

    #[test]
    fn test_let_variable() {
        let variable = GParser::parse(Rule::let_variable, "let generic_age = 22");
        println!("let: {:?}", variable);
        assert!(variable.is_ok());
    }

    #[test]
    fn test_man_variable() {
        let variable = GParser::parse(Rule::man_variable, "man \"Decay Rate\" as decay_rate = 22");
        println!("man: {:?}", variable);
        assert!(variable.is_ok());
    }

    #[test]
    fn test_const_variable() {
        let variable = GParser::parse(Rule::const_variable, "const generic_age = 22");
        println!("const: {:?}", variable);
        assert!(variable.is_ok());
    }

    #[test]
    fn test_use_statement() {
        let use_with_std = GParser::parse(Rule::import_pkg, "use \"math\"");
        println!("Use: {:?}", use_with_std);
        assert!(use_with_std.is_ok());
    }
}
