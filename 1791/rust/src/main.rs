impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        match edges[1].contains(&edges[0][0]) {
            true => edges[0][0],
            false => edges[0][1],
        }
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
