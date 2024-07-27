use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
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

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> Option<i32> {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(from_node_id, 0);
        heap.push(HeapNode {
            node_id: from_node_id,
            cost: 0,
            priority: self.heuristic(from_node_id, to_node_id),
        });

        while let Some(HeapNode { node_id, cost, .. }) = heap.pop() {
            if node_id == to_node_id {
                return Some(cost);
            }

            if cost > *distances.get(&node_id).unwrap_or(&i32::MAX) {
                continue;
            }

            if let Some(edges) = self.edges.get(&node_id) {
                for edge in edges {
                    let new_cost = cost.saturating_add(edge.weight);
                    let priority = new_cost.saturating_add(self.heuristic(edge.node_b_id, to_node_id));

                    if new_cost < *distances.get(&edge.node_b_id).unwrap_or(&i32::MAX) {
                        distances.insert(edge.node_b_id, new_cost);
                        heap.push(HeapNode {
                            node_id: edge.node_b_id,
                            cost: new_cost,
                            priority,
                        });
                    }
                }
            }
        }

        None
    }

    fn heuristic(&self, node_id: i32, goal_id: i32) -> i32 {
        let node = self.nodes.get(&node_id).unwrap();
        let goal = self.nodes.get(&goal_id).unwrap();
        ((node.x - goal.x).pow(2) + (node.y - goal.y).pow(2)) as f64.sqrt() as i32
    }
}

#[derive(Eq, PartialEq)]
struct HeapNode {
    node_id: i32,
    cost: i32,
    priority: i32,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
