// https://github.com/Geal/nom/blob/main/examples/s_expression.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
}

pub fn parse(s: &str) -> Expr {
    Expr::Number(42.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}