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

enum AddressesOrientation {
    /// Iterator will yield a row of values.
    Row,
    /// Iterator will yield a column of values.
    Column,
}

pub struct SheetRangeAddresses1D {
    fixed: i32,
    cur: i32,
    max: i32,
    orientation: AddressesOrientation,
}

impl Iterator for SheetRangeAddresses1D {
    type Item = SheetAddress;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > self.max {
            return None;
        }

        let result = SheetAddress {
            row: self.cur,
            col: self.fixed,
        };

        let result = match self.orientation {
            AddressesOrientation::Row => result.transpose(),
            AddressesOrientation::Column => result,
        };

        self.cur += 1;
        Some(result)
    }
}

pub struct SheetRangeAddresses2D {
    cur_row: i32,
    max_row: i32,
    start_col: i32,
    max_col: i32,
}

impl Iterator for SheetRangeAddresses2D {
    type Item = SheetRangeAddresses1D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row > self.max_row {
            return None;
        }

        let result = SheetRangeAddresses1D {
            fixed: self.cur_row,
            cur: self.start_col,
            max: self.max_col,
            orientation: AddressesOrientation::Row,
        };

        self.cur_row += 1;
        Some(result)
    }
}

pub enum SheetRangeShapedAddresses {
    Single { address: SheetAddress },
    Addresses1D(SheetRangeAddresses1D),
    Addresses2D(SheetRangeAddresses2D),
}

pub struct SheetRangeFlatAddresses {
    cur_col: i32,
    max_col: i32,
    min_col: i32,
    cur_row: i32,
    max_row: i32,
}

impl Iterator for SheetRangeFlatAddresses {
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

impl SheetRangeFlatAddresses {
    fn new(range: &SheetRange) -> Self {
        SheetRangeFlatAddresses {
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

    /// Returns a union type that can be used to iterate over the addresses in the range
    /// in a way specific to the shape of the range: a single address, a row or column of addresses,
    /// or a grid of addresses.
    pub fn addresses_shaped(&self) -> SheetRangeShapedAddresses {
        if self.start == self.end {
            SheetRangeShapedAddresses::Single {
                address: self.start.clone(),
            }
        } else if self.start.row == self.end.row || self.start.col == self.end.col {
            let orientation = if self.start.row == self.end.row {
                AddressesOrientation::Row
            } else {
                AddressesOrientation::Column
            };
            let (fixed, cur, max) = match orientation {
                AddressesOrientation::Row => (self.start.row, self.start.col, self.end.col),
                AddressesOrientation::Column => (self.start.col, self.start.row, self.end.row),
            };
            SheetRangeShapedAddresses::Addresses1D(SheetRangeAddresses1D {
                fixed,
                cur,
                max,
                orientation,
            })
        } else {
            SheetRangeShapedAddresses::Addresses2D(SheetRangeAddresses2D {
                cur_row: self.start.row,
                max_row: self.end.row,
                start_col: self.start.col,
                max_col: self.end.col,
            })
        }
    }

    /// Return an un-shaped iterator over all addresses in the range.
    pub fn addresses_flat(&self) -> SheetRangeFlatAddresses {
        SheetRangeFlatAddresses::new(self)
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
    fn test_addresses_flat() {
        let range = SheetRange {
            start: SheetAddress { row: 1, col: 1 },
            end: SheetAddress { row: 3, col: 3 },
        };

        assert_eq!(
            range.addresses_flat().collect::<Vec<_>>(),
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
            range.addresses_flat().collect::<Vec<_>>(),
            vec![SheetAddress { row: 1, col: 1 }]
        );
    }

    #[test]
    fn test_addresses_shaped() {
        // Single address
        let range = SheetRange {
            start: SheetAddress { row: 5, col: 5 },
            end: SheetAddress { row: 5, col: 5 },
        };

        assert!(match range.addresses_shaped() {
            SheetRangeShapedAddresses::Single { address } => {
                assert_eq!(address, SheetAddress { row: 5, col: 5 });
                true
            }
            _ => false,
        });

        // Row
        let range = SheetRange {
            start: SheetAddress { row: 5, col: 5 },
            end: SheetAddress { row: 5, col: 8 },
        };
        assert!(match range.addresses_shaped() {
            SheetRangeShapedAddresses::Addresses1D(iter) => {
                assert_eq!(
                    iter.collect::<Vec<_>>(),
                    vec![
                        SheetAddress { row: 5, col: 5 },
                        SheetAddress { row: 5, col: 6 },
                        SheetAddress { row: 5, col: 7 },
                        SheetAddress { row: 5, col: 8 },
                    ]
                );
                true
            }
            _ => false,
        });

        // Column
        let range = SheetRange {
            start: SheetAddress { row: 5, col: 5 },
            end: SheetAddress { row: 8, col: 5 },
        };
        assert!(match range.addresses_shaped() {
            SheetRangeShapedAddresses::Addresses1D(iter) => {
                assert_eq!(
                    iter.collect::<Vec<_>>(),
                    vec![
                        SheetAddress { row: 5, col: 5 },
                        SheetAddress { row: 6, col: 5 },
                        SheetAddress { row: 7, col: 5 },
                        SheetAddress { row: 8, col: 5 },
                    ]
                );
                true
            }
            _ => false,
        });

        // Grid
        let range = SheetRange {
            start: SheetAddress { row: 0, col: 0 },
            end: SheetAddress { row: 2, col: 2 },
        };
        assert!(match range.addresses_shaped() {
            SheetRangeShapedAddresses::Addresses2D(iter) => {
                assert_eq!(
                    iter.map(|row| row.collect::<Vec<_>>()).collect::<Vec<_>>(),
                    vec![
                        vec![
                            SheetAddress { row: 0, col: 0 },
                            SheetAddress { row: 0, col: 1 },
                            SheetAddress { row: 0, col: 2 },
                        ],
                        vec![
                            SheetAddress { row: 1, col: 0 },
                            SheetAddress { row: 1, col: 1 },
                            SheetAddress { row: 1, col: 2 },
                        ],
                        vec![
                            SheetAddress { row: 2, col: 0 },
                            SheetAddress { row: 2, col: 1 },
                            SheetAddress { row: 2, col: 2 },
                        ],
                    ]
                );
                true
            }
            _ => false,
        });
    }
}
