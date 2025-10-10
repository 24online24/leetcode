impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let n = potions.len();

        spells
            .into_iter()
            .map(|spell| {
                let min_potion_strength = ((success + spell as i64 - 1) / (spell as i64));
                (n - potions.partition_point(|&potion| (potion as i64) < min_potion_strength)) as i32
            })
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
