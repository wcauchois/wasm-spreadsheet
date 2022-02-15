// https://github.com/Geal/nom/blob/main/examples/s_expression.rs

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alpha1, char, digit1, multispace0, multispace1, none_of, one_of},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f32),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Expr>),
}

type ExprParseResult<'a> = IResult<&'a str, Expr, VerboseError<&'a str>>;

fn parse_number<'a>(input: &'a str) -> ExprParseResult<'a> {
    map(float, |n| Expr::Number(n))(input)
}

fn parse_string<'a>(input: &'a str) -> ExprParseResult<'a> {
    let esc = escaped(none_of("\\\""), '\\', one_of("\"n"));
    let esc_or_empty = alt((esc, tag("")));
    let (input, s) = delimited(tag("\""), esc_or_empty, tag("\""))(input)?;

    // Interpret escape sequences
    let interpreted_s = s.replace("\\\"", "\"").replace("\\n", "\n");
    Ok((input, Expr::String(interpreted_s)))
}

fn parse_expr<'a>(input: &'a str) -> ExprParseResult {
    preceded(
        multispace0,
        // alt((parse_number, parse_string, parse_symbol, parse_list)),
        alt((parse_number, parse_string)),
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
    fn test_parse_int_number() {
        assert_eq!(parse("42"), Ok(Expr::Number(42.0)));
    }

    #[test]
    fn test_parse_float_number() {
        assert_eq!(parse("1.234"), Ok(Expr::Number(1.234)));
    }

    #[test]
    fn test_parse_simple_string() {
        assert_eq!(parse(r#""hello""#), Ok(Expr::String("hello".into())));
    }

    #[test]
    fn test_parse_string_with_newline_escape() {
        assert_eq!(
            parse(r#""hello\nworld""#),
            Ok(Expr::String("hello\nworld".into()))
        );
    }

    #[test]
    fn test_parse_string_with_quote_escape() {
        assert_eq!(
            parse(r#""hello\"world""#),
            Ok(Expr::String("hello\"world".into()))
        );
    }
}
