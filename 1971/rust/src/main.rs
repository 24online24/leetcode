use std::collections::HashMap;

impl Solution {
    fn bfs(
        n: usize,
        neighbors: HashMap<i32, Vec<usize>>,
        source: usize,
        destination: usize,
    ) -> bool {
        let mut queue = vec![];
        let mut explored = vec![false; n as usize];
        explored[source] = true;
        queue.push(source);
        while queue.len() > 0 {
            // println!("{:?}", queue);
            let current_vertex = queue.remove(0);
            // println!("{:?} {}", current_vertex, destination);
            if current_vertex == destination {
                return true;
            }
            for neighbor in neighbors.get(&(current_vertex as i32)).unwrap() {
                if !explored[*neighbor] {
                    queue.push(*neighbor);
                    explored[*neighbor] = true;
                }
            }
        }
        false
    }

    fn dfs(
        n: usize,
        neighbors: HashMap<i32, Vec<usize>>,
        source: usize,
        destination: usize,
    ) -> bool {
        let mut stack = vec![];
        let mut explored = vec![false; n as usize];
        stack.push(source);
        while stack.len() > 0 {
            let current_vertex = stack.pop().unwrap();
            if current_vertex == destination {
                return true;
            }
            if !explored[current_vertex] {
                explored[current_vertex] = true;
                for neighbor in neighbors.get(&(current_vertex as i32)).unwrap() {
                    stack.push(*neighbor);
                }
            }
        }
        false
    }

    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        println!("----------------------------------");
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;

        let mut neighbors = HashMap::new();
        edges.iter().for_each(|edge| {
            neighbors
                .entry(edge[0])
                .or_insert(vec![])
                .push(edge[1] as usize);

            neighbors
                .entry(edge[1])
                .or_insert(vec![])
                .push(edge[0] as usize);
        });
        // println!("{:?}", neighbors);
        // Solution::bfs(n, neighbors, source, destination)
        Solution::dfs(n, neighbors, source, destination)
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
        true
    );

    assert_eq!(
        Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ),
        false
    );
}
