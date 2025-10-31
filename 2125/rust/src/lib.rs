impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter().map(|row| row.chars().filter(|c| *c == '1').count()).filter(|devices| *devices != 0)
            .collect::<Vec<_>>().windows(2).map(|w| w[0] * w[1]).sum()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
