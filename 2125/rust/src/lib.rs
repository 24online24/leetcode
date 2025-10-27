impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut lasers = 0;
        let mut last_row = 0;

        for row in bank {
            let devices = row.chars().filter(|c| *c == '1').count();
            if devices > 0 {
                lasers += last_row * devices;
                last_row = devices;
            }
        }

        lasers as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
