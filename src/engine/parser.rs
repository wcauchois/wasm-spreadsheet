// https://github.com/Geal/nom/blob/main/examples/s_expression.rs

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
}

fn parse_number<'a>(input: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map_res(digit1, |s: &str| s.parse::<f64>().map(|n| Expr::Number(n)))(input)
}

fn parse_expr<'a>(input: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    preceded(
        multispace0,
        // alt((parse_number, parse_string, parse_symbol, parse_list)),
        parse_number,
    )(input)
}

pub fn parse(src: &str) -> Result<Expr, String> {
    parse_expr(src)
        .map_err(|e: nom::Err<VerboseError<&str>>| format!("{:#?}", e))
        .map(|(_, exp)| exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse("42"), Ok(Expr::Number(42.0)));
    }
}
