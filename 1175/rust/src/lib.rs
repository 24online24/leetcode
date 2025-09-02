impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 45, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        let primes_in_permutation = primes.into_iter().position(|prime| prime > n).unwrap() as i32;
        let non_primes = n - primes_in_permutation;
        let mut result = 1;
        for i in 2..=primes_in_permutation as u64 {
            result *= i;
            result %= (10_u64.pow(9) + 7);
        }
        for i in 2..=non_primes as u64 {
            result *= i;
            result %= (10_u64.pow(9) + 7);
        }
        result as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
