// https://github.com/Geal/nom/blob/main/examples/s_expression.rs

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{anychar, char, multispace0, multispace1, none_of, one_of},
    character::is_alphanumeric,
    combinator::{cut, map, recognize, verify},
    error::{context, VerboseError},
    multi::{many1_count, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f32),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Expr>),
}

pub trait ExprVisitor {
    fn visit_number(&mut self, _num: f32) {}
    fn visit_string(&mut self, _s: &String) {}
    fn visit_symbol(&mut self, _sym: &String) {}
    fn visit_keyword(&mut self, _kw: &String) {}
}

impl Expr {
    pub fn walk(&self, visitor: &mut dyn ExprVisitor) {
        match self {
            Expr::Number(num) => visitor.visit_number(*num),
            Expr::String(s) => visitor.visit_string(&s),
            Expr::Symbol(sym) => visitor.visit_symbol(&sym),
            Expr::Keyword(kw) => visitor.visit_keyword(&kw),
            Expr::List(exprs) => {
                for expr in exprs {
                    expr.walk(visitor);
                }
            }
        }
    }
}

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;
type ExprParseResult<'a> = ParseResult<'a, Expr>;

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

fn parse_ident<'a>(input: &'a str) -> ParseResult<'a, &'a str> {
    fn is_valid_ident_char(c: char) -> bool {
        match c {
            c if is_alphanumeric(c as u8) => true,
            '+' => true,
            '-' => true,
            '/' => true,
            '_' => true,
            '*' => true,
            '>' => true,
            '<' => true,
            '|' => true,
            '&' => true,
            '.' => true,
            '=' => true,
            _ => false,
        }
    }

    recognize(many1_count(verify(anychar, |&c| is_valid_ident_char(c))))(input)
}

fn parse_symbol<'a>(input: &'a str) -> ExprParseResult<'a> {
    map(parse_ident, |i| Expr::Symbol(i.into()))(input)
}

fn parse_keyword<'a>(input: &'a str) -> ExprParseResult<'a> {
    map(preceded(tag(":"), parse_ident), |i| Expr::Keyword(i.into()))(input)
}

fn parse_expr<'a>(input: &'a str) -> ExprParseResult {
    let parse_list = map(
        delimited(
            char('('),
            separated_list0(multispace1, parse_expr),
            context("closing paren", cut(preceded(multispace0, char(')')))),
        ),
        |exprs| Expr::List(exprs),
    );

    preceded(
        multispace0,
        alt((
            parse_number,
            parse_string,
            // Order matters: number+string must go above symbol parsing, since
            // symbols can contain numbers or pretty much anything they want.
            parse_symbol,
            parse_keyword,
            parse_list,
        )),
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

    #[test]
    fn test_parse_ident() {
        assert_eq!(parse_ident("foo").map(|(_, i)| i), Ok("foo"));
        assert_eq!(parse_ident("foo_bar").map(|(_, i)| i), Ok("foo_bar"));
        assert_eq!(parse_ident("foo_123").map(|(_, i)| i), Ok("foo_123"));
        assert_eq!(parse_ident("foo_123").map(|(_, i)| i), Ok("foo_123"));
        assert_eq!(parse_ident("123").map(|(_, i)| i), Ok("123"));
        assert_eq!(parse_ident("+-").map(|(_, i)| i), Ok("+-"));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(parse("foobar"), Ok(Expr::Symbol("foobar".into())));
    }

    #[test]
    fn test_parse_keyword() {
        assert_eq!(parse(":foo"), Ok(Expr::Keyword("foo".into())));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            parse("(1 2)"),
            Ok(Expr::List(vec![Expr::Number(1.0), Expr::Number(2.0)]))
        );
        assert_eq!(
            parse("(hey (you :guy))"),
            Ok(Expr::List(vec![
                Expr::Symbol("hey".into()),
                Expr::List(vec![
                    Expr::Symbol("you".into()),
                    Expr::Keyword("guy".into()),
                ])
            ]))
        );
    }

    struct TestExprVisitor {
        visited_numbers: Vec<f32>,
        visited_strings: Vec<String>,
        visited_symbols: Vec<String>,
        visited_keywords: Vec<String>,
    }

    impl TestExprVisitor {
        fn new() -> Self {
            Self {
                visited_numbers: vec![],
                visited_strings: vec![],
                visited_symbols: vec![],
                visited_keywords: vec![],
            }
        }
    }

    impl ExprVisitor for TestExprVisitor {
        fn visit_number(&mut self, num: f32) {
            self.visited_numbers.push(num);
        }

        fn visit_string(&mut self, s: &String) {
            self.visited_strings.push(s.clone());
        }

        fn visit_symbol(&mut self, sym: &String) {
            self.visited_symbols.push(sym.clone());
        }

        fn visit_keyword(&mut self, kw: &String) {
            self.visited_keywords.push(kw.clone());
        }
    }

    #[test]
    fn test_expr_walk() {
        let expr = Expr::List(vec![
            Expr::List(vec![Expr::Number(42.0), Expr::String("hello".into())]),
            Expr::List(vec![
                Expr::String("world".into()),
                Expr::Symbol("baz".into()),
            ]),
            Expr::String("blah".into()),
        ]);

        let mut visitor = TestExprVisitor::new();
        expr.walk(&mut visitor);
        assert_eq!(visitor.visited_numbers, vec![42.0]);
        assert_eq!(
            visitor.visited_strings,
            vec!["hello".to_string(), "world".to_string(), "blah".to_string()]
        );
        assert_eq!(visitor.visited_symbols, vec!["baz".to_string()]);
    }
}
