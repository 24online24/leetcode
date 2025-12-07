impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];

        for query in queries {
            let (row1, col1, row2, col2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );
            for row in matrix.iter_mut().take(row2 + 1).skip(row1) {
                for element in row.iter_mut().take(col2 + 1).skip(col1) {
                    *element += 1;
                }
            }
        }

        matrix
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
