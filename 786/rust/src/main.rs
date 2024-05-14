use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut binary_heap = BinaryHeap::new();

        for i in 0..arr.len() {
            binary_heap.push((
                MinNonNan(arr[i] as f64 / arr[arr.len() - 1] as f64),
                (i, arr.len() - 1),
            ));
        }

        let mut k_copy = k - 1;
        while k_copy > 0 {
            let mut current = binary_heap.pop().unwrap().1;
            current.1 -= 1;
            binary_heap.push((
                MinNonNan(arr[current.0] as f64 / arr[current.1] as f64),
                (current.0, current.1),
            ));
            k_copy -= 1;
        }

        let result = binary_heap.peek().unwrap().1;
        vec![arr[result.0], arr[result.1]]
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3),
        vec![2, 5]
    );
    assert_eq!(
        Solution::kth_smallest_prime_fraction(vec![1, 7], 1),
        vec![1, 7]
    );
}
