use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut total_cost = f64::MAX;
        let mut current_total_quality = 0;
        let mut wage_to_quality_ratio = Vec::new();

        for i in 0..quality.len() {
            wage_to_quality_ratio.push((wage[i] as f64 / quality[i] as f64, quality[i]));
        }

        wage_to_quality_ratio.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut highest_quality_workers = BinaryHeap::new();

        for i in 0..quality.len() {
            highest_quality_workers.push(wage_to_quality_ratio[i].1);
            current_total_quality += wage_to_quality_ratio[i].1;

            if highest_quality_workers.len() > k as usize {
                current_total_quality -= highest_quality_workers.peek().unwrap();
                highest_quality_workers.pop();
            }

            if highest_quality_workers.len() == k as usize {
                total_cost = f64::min(
                    total_cost,
                    current_total_quality as f64 * wage_to_quality_ratio[i].0,
                )
            }
        }

        total_cost
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
        105.00000
    );
    assert_eq!(
        Solution::mincost_to_hire_workers(vec![4, 4, 4, 5], vec![13, 12, 13, 12], 2),
        26.00000
    );
}
