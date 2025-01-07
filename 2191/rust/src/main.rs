impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|num| {
            num.to_string()
                .chars()
                .map(|c| mapping[c.to_digit(10).unwrap() as usize])
                .fold(0, |acc, elem| acc * 10 + elem)
        });

        nums
    }
}

struct Solution;

fn main() {
    println!(
        "{:#?}",
        Solution::sort_jumbled(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        )
    );
}
