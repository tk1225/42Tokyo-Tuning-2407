use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap, BTreeMap};
use std::cmp::Ordering;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.entry(edge.node_a_id).or_default().push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges.entry(reverse_edge.node_a_id).or_default().push(reverse_edge);
    }

    fn heuristic(&self, node_a_id: i32, node_b_id: i32) -> i32 {
        let node_a = self.nodes.get(&node_a_id).unwrap();
        let node_b = self.nodes.get(&node_b_id).unwrap();
        (node_a.x - node_b.x).abs() + (node_a.y - node_b.y).abs()
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances: BTreeMap<i32, i32> = BTreeMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        distances.insert(from_node_id, 0);
        heap.push(State {
            cost: 0,
            position: from_node_id,
            estimated_cost: self.heuristic(from_node_id, to_node_id),
        });

        while let Some(State {
            cost,
            position,
            ..
        }) = heap.pop() {
            if position == to_node_id {
                return cost;
            }

            if cost > *distances.get(&position).unwrap_or(&i32::MAX) {
                continue;
            }

            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next_cost = cost + edge.weight;
                    if next_cost < *distances.get(&edge.node_b_id).unwrap_or(&i32::MAX) {
                        distances.insert(edge.node_b_id, next_cost);
                        heap.push(State {
                            cost: next_cost,
                            position: edge.node_b_id,
                            estimated_cost: next_cost + self.heuristic(edge.node_b_id, to_node_id),
                        });
                    }
                }
            }
        }

        i32::MAX // 経路が見つからなかった場合
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
    estimated_cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost) // 逆順にすることで最小ヒープとして扱う
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}