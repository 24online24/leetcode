impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        for i in 0..k / 2 {
            let top_row = x + i;
            let bottom_row = x + k - 1 - i;

            let (upper, lower) = grid.split_at_mut(bottom_row);
            let row_top = &mut upper[top_row];
            let row_bottom = &mut lower[0];

            row_top[y..y + k].swap_with_slice(&mut row_bottom[y..y + k]);
        }

        grid
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
