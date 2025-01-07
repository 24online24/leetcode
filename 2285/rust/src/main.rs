impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut frequency = vec![0; n as usize];
        roads.into_iter().for_each(|road| {
            frequency[road[0] as usize] += 1;
            frequency[road[1] as usize] += 1;
        });
        frequency.sort_unstable();
        frequency
            .into_iter()
            .enumerate()
            .fold(0, |total, (imp, freq)| total + (imp as i64 + 1) * freq)
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
