impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut cells = vec![vec![0; n as usize]; m as usize];
        for wall in walls {
            cells[wall[0] as usize][wall[1] as usize] = 1;
        }
        for guard in guards {
            let i = guard[0] as usize;
            let j = guard[1] as usize;
            cells[i][j] = 2;
            for j_left in (0..j).rev() {
                if cells[i][j_left] == 1 {
                    break;
                }
                if cells[i][j_left] == 2 {
                    continue;
                }
                cells[i][j_left] = 2;
            }
            for j_right in j + 1..n as usize {
                if cells[i][j_right] == 1 {
                    break;
                }
                if cells[i][j_right] == 2 {
                    continue;
                }
                cells[i][j_right] = 2;
            }
            for i_up in (0..i).rev() {
                if cells[i_up][j] == 1 {
                    break;
                }
                if cells[i_up][j] == 2 {
                    continue;
                }
                cells[i_up][j] = 2;
            }
            for i_down in i + 1..m as usize {
                if cells[i_down][j] == 1 {
                    break;
                }
                if cells[i_down][j] == 2 {
                    continue;
                }
                cells[i_down][j] = 2;
            }
        }
        cells.into_iter().flatten()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
