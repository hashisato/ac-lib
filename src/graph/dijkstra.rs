use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
}

#[derive(Debug)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

impl Eq for State {}

pub fn dijkstra(graph: &[Vec<Edge>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next_cost = cost + edge.cost;

            if next_cost < dist[edge.node] {
                dist[edge.node] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    position: edge.node,
                });
            }
        }
    }

    dist
}

pub fn dijkstra_with_path(graph: &[Vec<Edge>], start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut parent = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next_cost = cost + edge.cost;

            if next_cost < dist[edge.node] {
                dist[edge.node] = next_cost;
                parent[edge.node] = Some(position);
                heap.push(State {
                    cost: next_cost,
                    position: edge.node,
                });
            }
        }
    }

    (dist, parent)
}
