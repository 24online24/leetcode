impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut min_i = n;
        let mut min_j = m;
        let mut max_i = 0;
        let mut max_j = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &element) in row.iter().enumerate() {
                if element == 1 {
                    if i < min_i {
                        min_i = i;
                    }
                    if i > max_i {
                        max_i = i;
                    }
                    if j < min_j {
                        min_j = j;
                    }
                    if j > max_j {
                        max_j = j;
                    }
                }
            }
        }
        ((max_i + 1 - min_i) * (max_j + 1 - min_j)) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
