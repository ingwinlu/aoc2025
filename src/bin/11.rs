use petgraph::{algo::all_simple_paths, graph::DiGraph};
use std::{collections::HashMap, hash::RandomState};

advent_of_code::solution!(11);

#[derive(Debug)]
struct Problem {
    graph: DiGraph<String, ()>,
    node_map: HashMap<String, petgraph::prelude::NodeIndex>,
}

impl Problem {
    fn from_str(input: &str) -> Self {
        let mut graph = DiGraph::<String, ()>::new();
        let mut node_map = HashMap::new();

        let edges: Vec<(String, String)> = input
            .lines()
            .flat_map(|line| {
                let (source, dests) = line.split_once(": ").unwrap();
                dests
                    .split_whitespace()
                    .map(move |dest| (source.to_string(), dest.to_string()))
            })
            .collect();

        for (from, to) in &edges {
            if !node_map.contains_key(from) {
                let idx = graph.add_node(from.clone());
                node_map.insert(from.clone(), idx);
            }
            if !node_map.contains_key(to) {
                let idx = graph.add_node(to.clone());
                node_map.insert(to.clone(), idx);
            }
        }

        for (from, to) in edges {
            graph.add_edge(node_map[&from], node_map[&to], ());
        }
        Self { graph, node_map }
    }

    fn graph(&self) -> &DiGraph<String, ()> {
        &self.graph
    }

    fn node_idx_from_weight(&self, weight: &str) -> Option<&petgraph::prelude::NodeIndex> {
        self.node_map.get(weight)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let p = Problem::from_str(input);
    let start_idx = p.node_idx_from_weight("you").unwrap();
    let end_idx = p.node_idx_from_weight("out").unwrap();
    let paths =
        all_simple_paths::<Vec<_>, _, RandomState>(p.graph(), *start_idx, *end_idx, 0, None);
    Some(paths.count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let p = Problem::from_str(input);
    let graph = p.graph();

    fn count_paths_dag(
        graph: &DiGraph<String, ()>,
        from: petgraph::prelude::NodeIndex,
        to: petgraph::prelude::NodeIndex,
        memo: &mut HashMap<petgraph::prelude::NodeIndex, u64>,
    ) -> u64 {
        if from == to {
            return 1;
        }
        if let Some(&count) = memo.get(&from) {
            return count;
        }

        let count = graph
            .neighbors(from)
            .map(|v| count_paths_dag(graph, v, to, memo))
            .sum();

        memo.insert(from, count);
        count
    }

    let mut memo = HashMap::new();
    let mut count_paths = |from, to| {
        memo.clear();
        count_paths_dag(graph, from, to, &mut memo)
    };

    let svr_idx = *p.node_idx_from_weight("svr").unwrap();
    let out_idx = *p.node_idx_from_weight("out").unwrap();
    let dac_idx = *p.node_idx_from_weight("dac").unwrap();
    let fft_idx = *p.node_idx_from_weight("fft").unwrap();

    // Case 1: svr -> dac -> fft -> out
    let path1_count = count_paths(svr_idx, dac_idx)
        * count_paths(dac_idx, fft_idx)
        * count_paths(fft_idx, out_idx);

    // Case 2: svr -> fft -> dac -> out
    let path2_count = count_paths(svr_idx, fft_idx)
        * count_paths(fft_idx, dac_idx)
        * count_paths(dac_idx, out_idx);

    Some(path1_count + path2_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
