impl Solution {
    fn highest_prob_unvisited(probabilities: &Vec<f64>, unvisited: &Vec<bool>) -> Option<usize> {
        let mut max_prob = 0.0;
        let mut max_prob_index: Option<usize> = None;
        for i in 0..unvisited.len() {
            if unvisited[i] && probabilities[i] > 0.0 && probabilities[i] > max_prob {
                max_prob = probabilities[i];
                max_prob_index = Some(i);
            }
        }
        max_prob_index
    }

    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;

        let mut unvisited = vec![true; n];
        let mut nr_of_unvisited = n;
        let mut probabilities = vec![0_f64; n];
        probabilities[start] = 1.0;
        let mut current = start;
        while nr_of_unvisited > 0 {
            for (index, edge) in edges.iter().enumerate() {
                if edge[0] == current as i32 && unvisited[edge[1] as usize] {
                    if probabilities[current] * succ_prob[index] > probabilities[edge[1] as usize] {
                        probabilities[edge[1] as usize] = probabilities[current] * succ_prob[index];
                    }
                }

                if edge[1] == current as i32 && unvisited[edge[0] as usize] {
                    if probabilities[current] * succ_prob[index] > probabilities[edge[0] as usize] {
                        probabilities[edge[0] as usize] = probabilities[current] * succ_prob[index];
                    }
                }
            }
            unvisited[current] = false;
            if current == end {
                println!("Breaks at current == end");
                break;
            }
            match Solution::highest_prob_unvisited(&probabilities, &unvisited) {
                None => break,
                Some(node_index) => current = node_index,
            }
            nr_of_unvisited -= 1;
        }
        probabilities[end]
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_probability(
            5,
            vec![
                vec![1, 4],
                vec![2, 4],
                vec![0, 4],
                vec![0, 3],
                vec![0, 2],
                vec![2, 3]
            ],
            vec![0.37, 0.17, 0.93, 0.23, 0.39, 0.04],
            3,
            4
        ),
        0.21390
    );
}
