use std::collections::HashSet;

impl Solution {
    fn dfs(
        start: usize,
        neighbors: &Vec<Vec<usize>>,
        mut visited: HashSet<usize>,
    ) -> HashSet<usize> {
        let mut stack = Vec::with_capacity(neighbors.len());
        let mut length = 1;
        stack.push(start);
        visited.insert(start);
        while stack.len() > 0 {
            // println!("{:?}", stack);
            let current_vertex = stack.pop().unwrap();
            neighbors[current_vertex].iter().for_each(|&neighbor| {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                    visited.insert(neighbor);
                    length += 1;
                }
            });
        }
        visited
    }

    fn generate_neighbors(n: usize, connections: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut neighbors = vec![Vec::with_capacity(n); n];
        connections.into_iter().for_each(|edge| {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            neighbors[u].push(v);
            neighbors[v].push(u);
        });
        neighbors
    }

    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        if n > connections.len() + 1 {
            return -1;
        }

        let neighbors = Self::generate_neighbors(n, &connections);
        let mut start: usize = 0;
        let mut visited = HashSet::<usize>::new();

        let mut connected_components = 0;
        while visited.len() < n {
            while visited.contains(&start) {
                start += 1;
            }
            visited = Self::dfs(start, &neighbors, visited);
            connected_components += 1;
        }
        (connected_components - 1) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        1
    );

    assert_eq!(
        Solution::make_connected(
            100,
            vec![
                vec![17, 51],
                vec![33, 83],
                vec![53, 62],
                vec![25, 34],
                vec![35, 90],
                vec![29, 41],
                vec![14, 53],
                vec![40, 84],
                vec![41, 64],
                vec![13, 68],
                vec![44, 85],
                vec![57, 58],
                vec![50, 74],
                vec![20, 69],
                vec![15, 62],
                vec![25, 88],
                vec![4, 56],
                vec![37, 39],
                vec![30, 62],
                vec![69, 79],
                vec![33, 85],
                vec![24, 83],
                vec![35, 77],
                vec![2, 73],
                vec![6, 28],
                vec![46, 98],
                vec![11, 82],
                vec![29, 72],
                vec![67, 71],
                vec![12, 49],
                vec![42, 56],
                vec![56, 65],
                vec![40, 70],
                vec![24, 64],
                vec![29, 51],
                vec![20, 27],
                vec![45, 88],
                vec![58, 92],
                vec![60, 99],
                vec![33, 46],
                vec![19, 69],
                vec![33, 89],
                vec![54, 82],
                vec![16, 50],
                vec![35, 73],
                vec![19, 45],
                vec![19, 72],
                vec![1, 79],
                vec![27, 80],
                vec![22, 41],
                vec![52, 61],
                vec![50, 85],
                vec![27, 45],
                vec![4, 84],
                vec![11, 96],
                vec![0, 99],
                vec![29, 94],
                vec![9, 19],
                vec![66, 99],
                vec![20, 39],
                vec![16, 85],
                vec![12, 27],
                vec![16, 67],
                vec![61, 80],
                vec![67, 83],
                vec![16, 17],
                vec![24, 27],
                vec![16, 25],
                vec![41, 79],
                vec![51, 95],
                vec![46, 47],
                vec![27, 51],
                vec![31, 44],
                vec![0, 69],
                vec![61, 63],
                vec![33, 95],
                vec![17, 88],
                vec![70, 87],
                vec![40, 42],
                vec![21, 42],
                vec![67, 77],
                vec![33, 65],
                vec![3, 25],
                vec![39, 83],
                vec![34, 40],
                vec![15, 79],
                vec![30, 90],
                vec![58, 95],
                vec![45, 56],
                vec![37, 48],
                vec![24, 91],
                vec![31, 93],
                vec![83, 90],
                vec![17, 86],
                vec![61, 65],
                vec![15, 48],
                vec![34, 56],
                vec![12, 26],
                vec![39, 98],
                vec![1, 48],
                vec![21, 76],
                vec![72, 96],
                vec![30, 69],
                vec![46, 80],
                vec![6, 29],
                vec![29, 81],
                vec![22, 77],
                vec![85, 90],
                vec![79, 83],
                vec![6, 26],
                vec![33, 57],
                vec![3, 65],
                vec![63, 84],
                vec![77, 94],
                vec![26, 90],
                vec![64, 77],
                vec![0, 3],
                vec![27, 97],
                vec![66, 89],
                vec![18, 77],
                vec![27, 43]
            ]
        ),
        13
    );
}
