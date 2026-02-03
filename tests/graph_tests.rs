use ac_lib::graph::{
    bfs, bfs_with_callback, dfs, dfs_with_callback, dijkstra, dijkstra_with_path, Edge, UnionFind,
};

#[test]
fn test_dfs_basic() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let mut visited = vec![false; 4];
    let order = dfs(&graph, 0, &mut visited);

    assert_eq!(order.len(), 4);
    assert_eq!(order[0], 0);
    assert!(order.contains(&1));
    assert!(order.contains(&2));
    assert!(order.contains(&3));
}

#[test]
fn test_dfs_disconnected() {
    let graph = vec![vec![1], vec![0], vec![3], vec![2]];
    let mut visited = vec![false; 4];
    let order = dfs(&graph, 0, &mut visited);

    assert_eq!(order.len(), 2);
    assert_eq!(order, vec![0, 1]);
}

#[test]
fn test_dfs_with_callback_basic() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let mut visited = vec![false; 4];
    let mut result = Vec::new();

    dfs_with_callback(&graph, 0, &mut visited, &mut |node| {
        result.push(node);
    });

    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0);
}

#[test]
fn test_bfs_basic() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let mut visited = vec![false; 4];
    let order = bfs(&graph, 0, &mut visited);

    assert_eq!(order, vec![0, 1, 2, 3]);
}

#[test]
fn test_bfs_disconnected() {
    let graph = vec![vec![1], vec![0], vec![3], vec![2]];
    let mut visited = vec![false; 4];
    let order = bfs(&graph, 0, &mut visited);

    assert_eq!(order, vec![0, 1]);
}

#[test]
fn test_bfs_single_node() {
    let graph = vec![vec![]];
    let mut visited = vec![false; 1];
    let order = bfs(&graph, 0, &mut visited);

    assert_eq!(order, vec![0]);
}

#[test]
fn test_bfs_with_callback_basic() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let mut visited = vec![false; 4];
    let mut result = Vec::new();

    bfs_with_callback(&graph, 0, &mut visited, &mut |node| {
        result.push(node);
    });

    assert_eq!(result, vec![0, 1, 2, 3]);
}

#[test]
fn test_dijkstra_basic() {
    let graph = vec![
        vec![Edge { node: 1, cost: 4 }, Edge { node: 2, cost: 1 }],
        vec![Edge { node: 3, cost: 1 }],
        vec![Edge { node: 1, cost: 2 }, Edge { node: 3, cost: 5 }],
        vec![],
    ];
    let dist = dijkstra(&graph, 0);

    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 3);
    assert_eq!(dist[2], 1);
    assert_eq!(dist[3], 4);
}

#[test]
fn test_dijkstra_unreachable() {
    let graph = vec![
        vec![Edge { node: 1, cost: 1 }],
        vec![],
        vec![Edge { node: 3, cost: 1 }],
        vec![],
    ];
    let dist = dijkstra(&graph, 0);

    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
    assert_eq!(dist[2], usize::MAX);
    assert_eq!(dist[3], usize::MAX);
}

#[test]
fn test_dijkstra_single_node() {
    let graph = vec![vec![]];
    let dist = dijkstra(&graph, 0);

    assert_eq!(dist[0], 0);
}

#[test]
fn test_dijkstra_with_path() {
    let graph = vec![
        vec![Edge { node: 1, cost: 4 }, Edge { node: 2, cost: 1 }],
        vec![Edge { node: 3, cost: 1 }],
        vec![Edge { node: 1, cost: 2 }, Edge { node: 3, cost: 5 }],
        vec![],
    ];
    let (dist, parent) = dijkstra_with_path(&graph, 0);

    assert_eq!(dist[3], 4);
    assert_eq!(parent[3], Some(1));
    assert_eq!(parent[1], Some(2));
    assert_eq!(parent[2], Some(0));
}

#[test]
fn test_unionfind_new() {
    let mut uf = UnionFind::new(5);

    for i in 0..5 {
        assert_eq!(uf.find(i), i);
        assert_eq!(uf.size(i), 1);
    }
}

#[test]
fn test_unionfind_union() {
    let mut uf = UnionFind::new(5);

    assert!(uf.union(0, 1));
    assert!(uf.connected(0, 1));
    assert!(!uf.union(0, 1));
}

#[test]
fn test_unionfind_connected() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1);
    uf.union(1, 2);

    assert!(uf.connected(0, 2));
    assert!(!uf.connected(0, 3));
}

#[test]
fn test_unionfind_size() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1);
    uf.union(1, 2);

    assert_eq!(uf.size(0), 3);
    assert_eq!(uf.size(1), 3);
    assert_eq!(uf.size(2), 3);
    assert_eq!(uf.size(3), 1);
}

#[test]
fn test_unionfind_count_groups() {
    let mut uf = UnionFind::new(5);

    assert_eq!(uf.count_groups(), 5);

    uf.union(0, 1);
    assert_eq!(uf.count_groups(), 4);

    uf.union(2, 3);
    assert_eq!(uf.count_groups(), 3);

    uf.union(0, 2);
    assert_eq!(uf.count_groups(), 2);
}

#[test]
fn test_unionfind_all_connected() {
    let mut uf = UnionFind::new(4);

    uf.union(0, 1);
    uf.union(1, 2);
    uf.union(2, 3);

    assert_eq!(uf.count_groups(), 1);
    assert_eq!(uf.size(0), 4);
}

#[test]
fn test_unionfind_no_connections() {
    let mut uf = UnionFind::new(3);

    assert_eq!(uf.count_groups(), 3);
    assert!(!uf.connected(0, 1));
    assert!(!uf.connected(1, 2));
    assert!(!uf.connected(0, 2));
}
