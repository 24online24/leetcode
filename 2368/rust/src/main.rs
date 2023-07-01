impl Solution {
    fn bfs(n: usize, neighbors: Vec<Vec<usize>>, restricted: Vec<i32>) -> i32 {
        let mut stack = vec![0];
        let mut visited = vec![0; n as usize];
        restricted
            .into_iter()
            .for_each(|element| visited[element as usize] = 1);
        let mut length = 0;
        loop {
            length += 1;
            let current_vertex = stack.pop().unwrap();
            visited[current_vertex] = 1;
            neighbors[current_vertex].iter().for_each(|neighbor| {
                if visited[*neighbor] == 0 {
                    stack.push(*neighbor);
                }
            });
            if stack.len() <= 0 {
                break length;
            }
        }
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![Vec::with_capacity(n); n];
        neighbors.insert(0, vec![]);
        edges.into_iter().for_each(|edge| {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            neighbors[u].push(v);
            neighbors[v].push(u);
        });
        Solution::bfs(n, neighbors, restricted)
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::reachable_nodes(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![3, 1],
                vec![4, 0],
                vec![0, 5],
                vec![5, 6]
            ],
            vec![4, 5]
        ),
        4
    )
}
