impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let l = grid.len();
        let mut side_area = 0;
        let mut bottom_area = 0;
        let mut element;
        for i in 0..l {
            for j in 0..l {
                element = grid[i][j];
                if element > 0 {
                    bottom_area += 1;
                    if i == 0 {
                        side_area += element;
                    }
                    if j == 0 {
                        side_area += element;
                    }
                }
                if i == l - 1 {
                    side_area += element;
                } else {
                    side_area += (element - grid[i + 1][j]).abs();
                }
                if j == l - 1 {
                    side_area += element;
                } else {
                    side_area += (element - grid[i][j + 1]).abs();
                }
            }
        }
        2 * bottom_area + side_area
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(
        Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        32
    );
    assert_eq!(
        Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 1, 2]]),
        46
    );
}
