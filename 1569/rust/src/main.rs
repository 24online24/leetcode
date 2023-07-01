use std::convert::TryInto;

const MOD: u128 = 10_u128.pow(9) + 7;

impl Solution {
    fn combinations(a: u128, b: u128) -> u128 {
        if a > b {
            ((a + 1)..=(a + b)).product::<u128>() / (1..=b).product::<u128>() % MOD
        } else {
            ((b + 1)..=(a + b)).product::<u128>() / (1..=a).product::<u128>() % MOD
        }
    }

    fn ways(nums: Vec<i32>) -> u128 {
        if nums.len() <= 2 {
            return 1;
        }

        let root = nums[0];
        let left: Vec<i32> = nums.clone().into_iter().filter(|&num| num < root).collect();
        let right: Vec<i32> = nums.clone().into_iter().filter(|&num| num > root).collect();

        let ans_l = Solution::ways(left.clone()) % MOD;
        let ans_r = Solution::ways(right.clone()) % MOD;
        println!("Before combinations");
        let comb = Solution::combinations(
            left.len().try_into().unwrap(),
            right.len().try_into().unwrap(),
        ) % MOD;
        println!("After combinations");
        println!("{:?}", nums);
        println!("{}, {}, {}", ans_l, ans_r, comb);
        let lr_mult = (ans_l * ans_r) % MOD;
        println!("{}", lr_mult);
        let final_mult = (lr_mult * comb) % MOD;
        println!("{}", final_mult);
        final_mult
    }

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        ((Solution::ways(nums) - 1) % (MOD)).try_into().unwrap()
    }
}

struct Solution;

fn main() {
    // test(vec![1, 2, 3], 0);

    // test(
    //     vec![
    //         31, 23, 14, 24, 15, 12, 25, 28, 5, 35, 17, 6, 9, 11, 1, 27, 18, 20, 2, 3, 33, 10, 13,
    //         4, 7, 36, 32, 29, 8, 30, 26, 19, 34, 22, 21, 16,
    //     ],
    //     936157466,
    // );

    test(
        vec![
            74, 24, 70, 11, 6, 4, 59, 9, 36, 82, 80, 30, 46, 31, 22, 34, 8, 69, 32, 57, 18, 21, 37,
            83, 55, 38, 41, 72, 48, 65, 27, 60, 73, 58, 68, 50, 16, 77, 75, 20, 81, 3, 61, 13, 10,
            29, 62, 49, 12, 66, 39, 45, 28, 40, 42, 52, 78, 56, 44, 17, 14, 67, 35, 26, 19, 5, 63,
            51, 43, 23, 79, 2, 54, 47, 76, 53, 7, 25, 64, 33, 1, 15, 71,
        ],
        901891111,
    );
}

fn test(nums: Vec<i32>, result: i32) {
    assert_eq!(Solution::num_of_ways(nums), result);
}
