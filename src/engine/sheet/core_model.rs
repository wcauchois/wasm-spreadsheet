#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct SheetAddress {
    pub row: i32,
    pub col: i32,
}

impl SheetAddress {
    pub fn transpose(&self) -> Self {
        Self {
            row: self.col,
            col: self.row,
        }
    }
}
