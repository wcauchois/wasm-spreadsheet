use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    error::VerboseError,
    sequence::{separated_pair, tuple},
};

use super::SheetAddress;

use crate::error::{AppError, AppResult};
use crate::parser::ParseResult;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct SheetRange {
    pub start: SheetAddress,
    pub end: SheetAddress,
}

#[derive(Debug)]
pub struct SheetRangeAddresses {
    cur_col: i32,
    max_col: i32,
    min_col: i32,
    cur_row: i32,
    max_row: i32,
}

impl Iterator for SheetRangeAddresses {
    type Item = SheetAddress;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row > self.max_row {
            return None;
        }

        let result = SheetAddress {
            row: self.cur_row,
            col: self.cur_col,
        };

        self.cur_col += 1;
        if self.cur_col > self.max_col {
            self.cur_col = self.min_col;
            self.cur_row += 1;
        }

        Some(result)
    }
}

impl SheetRangeAddresses {
    fn new(range: &SheetRange) -> Self {
        SheetRangeAddresses {
            cur_col: range.start.col,
            min_col: range.start.col,
            max_col: range.end.col,
            cur_row: range.start.row,
            max_row: range.end.row,
        }
    }
}

fn parse_sheet_column<'a>(input: &'a str) -> ParseResult<'a, i32> {
    let (input, s) = alpha1(input)?;
    // TODO: Support multi-char columns like "AA", "BB", etc.
    Ok((
        input,
        s.chars().next().unwrap().to_lowercase().next().unwrap() as i32 - 'a' as i32,
    ))
}

fn parse_integer<'a>(input: &'a str) -> ParseResult<'a, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_sheet_address<'a>(input: &'a str) -> ParseResult<'a, SheetAddress> {
    let (input, (col, row_number)) = tuple((parse_sheet_column, parse_integer))(input)?;
    let address = SheetAddress {
        row: row_number - 1,
        col,
    };
    Ok((input, address))
}

fn parse_sheet_range<'a>(input: &'a str) -> ParseResult<'a, SheetRange> {
    fn singular_range<'a>(input: &'a str) -> ParseResult<'a, SheetRange> {
        map(parse_sheet_address, |addr| SheetRange {
            start: addr.clone(),
            end: addr,
        })(input)
    }

    fn composite_range<'a>(input: &'a str) -> ParseResult<'a, SheetRange> {
        let (input, (start, end)) =
            separated_pair(parse_sheet_address, tag("-"), parse_sheet_address)(input)?;
        Ok((input, SheetRange { start, end }))
    }

    alt((composite_range, singular_range))(input)
}

impl SheetRange {
    fn is_valid(&self) -> bool {
        self.end.row >= self.start.row && self.end.col >= self.start.col
    }

    pub fn parse(input: &str) -> AppResult<Self> {
        let range = parse_sheet_range(input)
            .map_err(|e: nom::Err<VerboseError<&str>>| AppError::new(format!("{:#?}", e)))
            .map(|(_, result)| result)?;
        if !range.is_valid() {
            return Err(AppError::new(
                "Invalid range: end must be bottom-right from start",
            ));
        }
        Ok(range)
    }

    pub fn addresses(&self) -> SheetRangeAddresses {
        SheetRangeAddresses::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sheet_column() {
        assert_eq!(parse_sheet_column("a"), Ok(("", 0)));
        assert_eq!(parse_sheet_column("A"), Ok(("", 0)));
        assert_eq!(parse_sheet_column("c"), Ok(("", 2)));
    }

    #[test]
    fn test_parse_sheet_address() {
        assert_eq!(
            parse_sheet_address("a2"),
            Ok(("", SheetAddress { row: 1, col: 0 }))
        );
    }

    #[test]
    fn test_parse_sheet_range() {
        assert_eq!(
            parse_sheet_range("a2"),
            Ok((
                "",
                SheetRange {
                    start: SheetAddress { row: 1, col: 0 },
                    end: SheetAddress { row: 1, col: 0 },
                }
            ))
        );

        assert_eq!(
            parse_sheet_range("a2-c6"),
            Ok((
                "",
                SheetRange {
                    start: SheetAddress { row: 1, col: 0 },
                    end: SheetAddress { row: 5, col: 2 },
                }
            ))
        );
    }

    #[test]
    fn test_sheet_range_addresses() {
        let range = SheetRange {
            start: SheetAddress { row: 1, col: 1 },
            end: SheetAddress { row: 3, col: 3 },
        };

        assert_eq!(
            range.addresses().collect::<Vec<_>>(),
            vec![
                SheetAddress { row: 1, col: 1 },
                SheetAddress { row: 1, col: 2 },
                SheetAddress { row: 1, col: 3 },
                SheetAddress { row: 2, col: 1 },
                SheetAddress { row: 2, col: 2 },
                SheetAddress { row: 2, col: 3 },
                SheetAddress { row: 3, col: 1 },
                SheetAddress { row: 3, col: 2 },
                SheetAddress { row: 3, col: 3 },
            ]
        );

        // Same start as end
        let range = SheetRange {
            start: SheetAddress { row: 1, col: 1 },
            end: SheetAddress { row: 1, col: 1 },
        };

        assert_eq!(
            range.addresses().collect::<Vec<_>>(),
            vec![SheetAddress { row: 1, col: 1 }]
        );
    }
}
