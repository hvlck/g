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
    }

    #[test]
    fn test_expression() {
        let exp = GParser::parse(Rule::expression, "1+1");
        println!("Expression: {:?}", exp);
    }
}
