impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let start = 0;
        let end = 0;
        let 
        for num in nums {
            if num == 1 {
                size += 1;
                last_was_0 = false;
            } else {
                if last_was_0 {
                    if size > biggest_size {
                        biggest_size = size;
                    }
                    size = 0;
                }
                last_was_0 = true;
            }
        }
        if size > biggest_size {
            biggest_size = size;
        }
        biggest_size
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
