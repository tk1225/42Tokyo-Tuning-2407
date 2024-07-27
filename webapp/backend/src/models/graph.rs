use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub target: usize,
    pub weight: f64,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<Edge>>,
    node_lookup: HashMap<usize, usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_lookup: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: usize, x: f64, y: f64) {
        let index = self.nodes.len();
        self.nodes.push(Node { id, x, y });
        self.edges.push(Vec::new());
        self.node_lookup.insert(id, index);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        let from_index = *self.node_lookup.get(&from).expect("Invalid 'from' node");
        let to_index = *self.node_lookup.get(&to).expect("Invalid 'to' node");
        self.edges[from_index].push(Edge { target: to_index, weight });
        self.edges[to_index].push(Edge { target: from_index, weight });
    }

    pub fn shortest_path(&self, from: usize, to: usize) -> Option<(f64, Vec<usize>)> {
        let from_index = *self.node_lookup.get(&from)?;
        let to_index = *self.node_lookup.get(&to)?;

        let mut dist: Vec<_> = (0..self.nodes.len()).map(|_| f64::INFINITY).collect();
        let mut prev: Vec<_> = (0..self.nodes.len()).map(|_| None).collect();
        let mut heap = BinaryHeap::new();

        dist[from_index] = 0.0;
        heap.push(State {
            cost: 0.0,
            position: from_index,
            f_score: self.heuristic(from_index, to_index),
        });

        while let Some(State { cost, position, .. }) = heap.pop() {
            if position == to_index {
                let mut path = Vec::new();
                let mut current = position;
                while let Some(p) = prev[current] {
                    path.push(self.nodes[current].id);
                    current = p;
                }
                path.push(from);
                path.reverse();
                return Some((cost, path));
            }

            if cost > dist[position] {
                continue;
            }

            for edge in &self.edges[position] {
                let next = State {
                    cost: cost + edge.weight,
                    position: edge.target,
                    f_score: cost + edge.weight + self.heuristic(edge.target, to_index),
                };

                if next.cost < dist[next.position] {
                    dist[next.position] = next.cost;
                    prev[next.position] = Some(position);
                    heap.push(next);
                }
            }
        }

        None
    }

    fn heuristic(&self, from: usize, to: usize) -> f64 {
        let dx = self.nodes[from].x - self.nodes[to].x;
        let dy = self.nodes[from].y - self.nodes[to].y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn parallel_shortest_paths(&self, pairs: &[(usize, usize)]) -> Vec<Option<(f64, Vec<usize>)>> {
        let arc_self = Arc::new(self);
        pairs.par_iter()
            .map(|&(from, to)| arc_self.shortest_path(from, to))
            .collect()
    }

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.node_lookup.get(&id).map(|&index| &self.nodes[index])
    }

    pub fn get_edges(&self, id: usize) -> Option<&[Edge]> {
        self.node_lookup.get(&id).map(|&index| &self.edges[index][..])
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: f64,
    position: usize,
    f_score: f64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();
        graph.add_node(1, 0.0, 0.0);
        graph.add_node(2, 1.0, 1.0);
        graph.add_node(3, 2.0, 0.0);
        graph.add_edge(1, 2, 1.414);
        graph.add_edge(2, 3, 1.414);
        graph.add_edge(1, 3, 2.0);

        let (cost, path) = graph.shortest_path(1, 3).unwrap();
        assert_eq!(path, vec![1, 3]);
        assert!((cost - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_parallel_shortest_paths() {
        let mut graph = Graph::new();
        for i in 0..1000 {
            graph.add_node(i, i as f64, (i % 10) as f64);
        }
        for i in 0..999 {
            graph.add_edge(i, i + 1, 1.0);
        }

        let pairs = vec![(0, 999), (0, 500), (250, 750), (100, 900)];
        let results = graph.parallel_shortest_paths(&pairs);

        assert_eq!(results.len(), 4);
        assert_eq!(results[0].as_ref().map(|(cost, _)| *cost as usize), Some(999));
    }
}
