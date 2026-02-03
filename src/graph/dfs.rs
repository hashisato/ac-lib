pub fn dfs(graph: &[Vec<usize>], start: usize, visited: &mut [bool]) -> Vec<usize> {
    let mut order = Vec::new();
    dfs_helper(graph, start, visited, &mut order);
    order
}

fn dfs_helper(graph: &[Vec<usize>], node: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    visited[node] = true;
    order.push(node);

    for &neighbor in &graph[node] {
        if !visited[neighbor] {
            dfs_helper(graph, neighbor, visited, order);
        }
    }
}

pub fn dfs_with_callback<F>(
    graph: &[Vec<usize>],
    start: usize,
    visited: &mut [bool],
    callback: &mut F,
) where
    F: FnMut(usize),
{
    visited[start] = true;
    callback(start);

    for &neighbor in &graph[start] {
        if !visited[neighbor] {
            dfs_with_callback(graph, neighbor, visited, callback);
        }
    }
}
