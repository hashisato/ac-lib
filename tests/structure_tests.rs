use ac_lib::structure::{FenwickTree, SegmentTree};

#[test]
fn test_segtree_new() {
    let arr = vec![1, 3, 5, 7, 9];
    let segtree = SegmentTree::new(&arr);

    for i in 0..arr.len() {
        assert_eq!(segtree.get(i), arr[i]);
    }
}

#[test]
fn test_segtree_query_full_range() {
    let arr = vec![1, 3, 5, 7, 9];
    let segtree = SegmentTree::new(&arr);

    assert_eq!(segtree.query(0, 5), 25);
}

#[test]
fn test_segtree_query_partial_range() {
    let arr = vec![1, 3, 5, 7, 9];
    let segtree = SegmentTree::new(&arr);

    assert_eq!(segtree.query(1, 4), 15);
    assert_eq!(segtree.query(0, 3), 9);
    assert_eq!(segtree.query(2, 5), 21);
}

#[test]
fn test_segtree_query_single_element() {
    let arr = vec![1, 3, 5, 7, 9];
    let segtree = SegmentTree::new(&arr);

    assert_eq!(segtree.query(2, 3), 5);
    assert_eq!(segtree.query(0, 1), 1);
    assert_eq!(segtree.query(4, 5), 9);
}

#[test]
fn test_segtree_update() {
    let arr = vec![1, 3, 5, 7, 9];
    let mut segtree = SegmentTree::new(&arr);

    segtree.update(2, 10);
    assert_eq!(segtree.get(2), 10);
    assert_eq!(segtree.query(0, 5), 30);
    assert_eq!(segtree.query(1, 4), 20);
}

#[test]
fn test_segtree_multiple_updates() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut segtree = SegmentTree::new(&arr);

    segtree.update(0, 10);
    segtree.update(4, 50);

    assert_eq!(segtree.query(0, 5), 69);
}

#[test]
fn test_segtree_empty_range() {
    let arr = vec![1, 3, 5, 7, 9];
    let segtree = SegmentTree::new(&arr);

    assert_eq!(segtree.query(2, 2), 0);
}

#[test]
fn test_segtree_single_element_array() {
    let arr = vec![42];
    let mut segtree = SegmentTree::new(&arr);

    assert_eq!(segtree.query(0, 1), 42);
    segtree.update(0, 100);
    assert_eq!(segtree.query(0, 1), 100);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_segtree_update_out_of_bounds() {
    let arr = vec![1, 2, 3];
    let mut segtree = SegmentTree::new(&arr);
    segtree.update(5, 10);
}

#[test]
#[should_panic(expected = "Invalid range")]
fn test_segtree_query_invalid_range() {
    let arr = vec![1, 2, 3];
    let segtree = SegmentTree::new(&arr);
    segtree.query(2, 1);
}

// FenwickTree Tests
#[test]
fn test_fenwick_new() {
    let ft = FenwickTree::new(5);

    for i in 0..5 {
        assert_eq!(ft.get(i), 0);
    }
}

#[test]
fn test_fenwick_from_vec() {
    let arr = vec![1, 3, 5, 7, 9];
    let ft = FenwickTree::from_vec(&arr);

    for i in 0..arr.len() {
        assert_eq!(ft.get(i), arr[i]);
    }
}

#[test]
fn test_fenwick_add() {
    let mut ft = FenwickTree::new(5);

    ft.add(0, 3);
    ft.add(1, 5);
    ft.add(2, 2);

    assert_eq!(ft.get(0), 3);
    assert_eq!(ft.get(1), 5);
    assert_eq!(ft.get(2), 2);
}

#[test]
fn test_fenwick_sum() {
    let arr = vec![1, 3, 5, 7, 9];
    let ft = FenwickTree::from_vec(&arr);

    assert_eq!(ft.sum(0), 1);
    assert_eq!(ft.sum(1), 4);
    assert_eq!(ft.sum(2), 9);
    assert_eq!(ft.sum(3), 16);
    assert_eq!(ft.sum(4), 25);
}

#[test]
fn test_fenwick_range_sum() {
    let arr = vec![1, 3, 5, 7, 9];
    let ft = FenwickTree::from_vec(&arr);

    assert_eq!(ft.range_sum(0, 4), 25);
    assert_eq!(ft.range_sum(1, 3), 15);
    assert_eq!(ft.range_sum(2, 2), 5);
    assert_eq!(ft.range_sum(0, 2), 9);
}

#[test]
fn test_fenwick_set() {
    let arr = vec![1, 3, 5, 7, 9];
    let mut ft = FenwickTree::from_vec(&arr);

    ft.set(2, 10);
    assert_eq!(ft.get(2), 10);
    assert_eq!(ft.sum(4), 30);
}

#[test]
fn test_fenwick_multiple_operations() {
    let mut ft = FenwickTree::new(5);

    ft.add(0, 1);
    ft.add(1, 2);
    ft.add(2, 3);
    ft.add(3, 4);
    ft.add(4, 5);

    assert_eq!(ft.range_sum(0, 4), 15);

    ft.set(2, 10);
    assert_eq!(ft.range_sum(0, 4), 22);

    ft.add(1, 5);
    assert_eq!(ft.range_sum(0, 4), 27);
}

#[test]
fn test_fenwick_add_negative() {
    let mut ft = FenwickTree::new(5);

    ft.add(0, 10);
    ft.add(0, -5);

    assert_eq!(ft.get(0), 5);
}

#[test]
fn test_fenwick_single_element() {
    let mut ft = FenwickTree::new(1);

    ft.add(0, 42);
    assert_eq!(ft.sum(0), 42);
    assert_eq!(ft.get(0), 42);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_fenwick_add_out_of_bounds() {
    let mut ft = FenwickTree::new(3);
    ft.add(5, 10);
}

#[test]
#[should_panic(expected = "Invalid range")]
fn test_fenwick_range_sum_invalid() {
    let ft = FenwickTree::new(5);
    ft.range_sum(3, 1);
}

#[test]
fn test_segtree_vs_fenwick_consistency() {
    let arr = vec![1, 3, 5, 7, 9];

    let segtree = SegmentTree::new(&arr);
    let ft = FenwickTree::from_vec(&arr);

    assert_eq!(segtree.query(0, 5), ft.sum(4));
    assert_eq!(segtree.query(1, 4), ft.range_sum(1, 3));
    assert_eq!(segtree.query(2, 3), ft.range_sum(2, 2));
}

#[test]
fn test_large_array() {
    let arr: Vec<i64> = (1..=1000).collect();

    let segtree = SegmentTree::new(&arr);
    let ft = FenwickTree::from_vec(&arr);

    assert_eq!(segtree.query(0, 1000), 500500);
    assert_eq!(ft.sum(999), 500500);
}
