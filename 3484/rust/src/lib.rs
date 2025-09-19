struct Spreadsheet {
    grid: Vec<[i32; 26]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn parse_cell(cell: &str) -> (usize, usize) {
        let col = &cell[..1];
        let row = &cell[1..];
        (row.parse().unwrap(), (cell.as_bytes()[0] - b'A') as usize)
    }

    fn get_numeric_value(&self, value: &str) -> i32 {
        if let Ok(numeric_value) = value.parse() {
            return numeric_value;
        }
        let (x, y) = Self::parse_cell(value);
        return self.grid[x][y];
    }

    fn new(rows: i32) -> Self {
        Spreadsheet {
            grid: vec![[0; 26]; (rows + 1) as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let (x, y) = Self::parse_cell(&cell);
        self.grid[x][y] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let (x, y) = Self::parse_cell(&cell);
        self.grid[x][y] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let values: Vec<&str> = formula.split(['=', '+']).skip(1).collect();
        Self::get_numeric_value(self, values[0]) + Self::get_numeric_value(self, values[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut spreadsheet = Spreadsheet::new(3);
        assert_eq!(12, spreadsheet.get_value("=5+7".to_owned()));
        spreadsheet.set_cell("A1".to_owned(), 10);
        assert_eq!(16, spreadsheet.get_value("=A1+6".to_owned()));
        spreadsheet.set_cell("B2".to_owned(), 15);
        assert_eq!(25, spreadsheet.get_value("=A1+B2".to_owned()));
        spreadsheet.reset_cell("A1".to_owned());
        assert_eq!(15, spreadsheet.get_value("=A1+B2".to_owned()));
    }

    #[test]
    fn example_2() {
        let mut spreadsheet = Spreadsheet::new(458);
        assert_eq!(10272, spreadsheet.get_value("=O126+10272".to_owned()));
    }
}
