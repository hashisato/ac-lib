use std::collections::VecDeque;

pub fn bfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    order
}

pub fn bfs_with_callback<F>(
    graph: &Vec<Vec<usize>>,
    start: usize,
    visited: &mut Vec<bool>,
    callback: &mut F,
) where
    F: FnMut(usize),
{
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        callback(node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
}
