use std::collections::HashMap;

impl Solution {
    fn create_data(
        c: i32,
        connections: Vec<Vec<i32>>,
    ) -> (Vec<(i32, bool)>, HashMap<i32, Vec<i32>>) {
        let mut station_data: Vec<(i32, bool)> = (1..=c).map(|stat| (stat, true)).collect();
        for connection in connections {
            let parent1 = station_data[connection[0] as usize - 1].0;
            let parent2 = station_data[connection[1] as usize - 1].0;
            let parent = i32::min(parent1, parent2);
            station_data[connection[0] as usize - 1].0 = parent;
            station_data[connection[1] as usize - 1].0 = parent;
            for i in 0..c as usize {
                if station_data[i].0 == parent1 || station_data[i].0 == parent2 {
                    station_data[i].0 = parent;
                }
            }
        }

        let mut power_grids: HashMap<i32, Vec<i32>> = HashMap::with_capacity(c as usize);
        for (station, (grid, _)) in station_data.iter().enumerate() {
            power_grids
                .entry(*grid)
                .or_default()
                .push(station as i32 + 1);
        }

        return (station_data, power_grids);
    }

    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut station_data, power_grids) = Self::create_data(c, connections);

        let mut query_results = Vec::new();

        for query in queries {
            match query[0] {
                1 => {
                    let (grid, online) = station_data[query[1] as usize - 1];
                    if online {
                        query_results.push(query[1]);
                    } else {
                        let mut found = false;
                        for station in power_grids[&grid].clone() {
                            if station_data[station as usize - 1].1 {
                                found = true;
                                query_results.push(station);
                                break;
                            }
                        }
                        if !found {
                            query_results.push(-1);
                        }
                    }
                }
                2 => {
                    station_data[query[1] as usize - 1].1 = false;
                }
                _ => {}
            }
        }

        query_results
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_data_1() {
        let (station_data, power_grids) =
            Solution::create_data(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]);
        assert_eq!(
            vec![(1, true), (1, true), (1, true), (1, true), (1, true)],
            station_data
        );
        assert_eq!(HashMap::from([(1, vec![1, 2, 3, 4, 5])]), power_grids);
    }

    #[test]
    fn create_data_2() {
        let (station_data, power_grids) = Solution::create_data(3, vec![]);
        assert_eq!(vec![(1, true), (2, true), (3, true)], station_data);
        assert_eq!(
            HashMap::from([(1, vec![1]), (2, vec![2]), (3, vec![3])]),
            power_grids
        );
    }

    #[test]
    fn create_data_3() {
        let (station_data, power_grids) =
            Solution::create_data(3, vec![vec![3, 2], vec![1, 3], vec![2, 1]]);
        assert_eq!(vec![(1, true), (1, true), (1, true)], station_data);
        assert_eq!(HashMap::from([(1, vec![1, 2, 3])]), power_grids);
    }

    #[test]
    fn create_data_4() {
        let (station_data, power_grids) = Solution::create_data(4, vec![vec![3, 4], vec![4, 2]]);
        assert_eq!(
            vec![(1, true), (2, true), (2, true), (2, true)],
            station_data
        );
        assert_eq!(
            HashMap::from([(1, vec![1]), (2, vec![2, 3, 4])]),
            power_grids
        );
    }

    #[test]
    fn create_data_5() {
        let (station_data, power_grids) =
            Solution::create_data(5, vec![vec![2, 3], vec![2, 4], vec![4, 1], vec![5, 1]]);
        assert_eq!(
            vec![(1, true), (1, true), (1, true), (1, true), (1, true)],
            station_data
        );
        assert_eq!(HashMap::from([(1, vec![1, 2, 3, 4, 5])]), power_grids);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            vec![3, 2, 3],
            Solution::process_queries(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, -1],
            Solution::process_queries(3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]])
        );
    }
}
