impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        (0..grid.len() - 2)
            .map(|i| {
                (0..grid.len() - 2)
                    .map(|j| {
                        (0..9)
                            .map(|delta| grid[i + delta / 3][j + delta % 3])
                            .max()
                            .unwrap()
                    })
                    .collect()
            })
            .collect()
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
    let x: Vec<_> = (0..9).map(|i| i).collect();
    println!("{:?}", x);
}
