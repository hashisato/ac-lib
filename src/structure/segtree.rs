pub struct SegmentTree {
    size: usize,
    tree: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let size = n.next_power_of_two();
        let mut tree = vec![0; size * 2];

        tree[size..(n + size)].copy_from_slice(&arr[..n]);

        for i in (1..size).rev() {
            tree[i] = tree[i * 2] + tree[i * 2 + 1];
        }

        SegmentTree { size, tree, n }
    }

    pub fn update(&mut self, idx: usize, value: i64) {
        assert!(idx < self.n, "Index out of bounds");

        let mut pos = self.size + idx;
        self.tree[pos] = value;

        while pos > 1 {
            pos /= 2;
            self.tree[pos] = self.tree[pos * 2] + self.tree[pos * 2 + 1];
        }
    }

    pub fn query(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r <= self.n, "Invalid range");

        let mut left = self.size + l;
        let mut right = self.size + r;
        let mut sum = 0;

        while left < right {
            if left % 2 == 1 {
                sum += self.tree[left];
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                sum += self.tree[right];
            }
            left /= 2;
            right /= 2;
        }

        sum
    }

    pub fn get(&self, idx: usize) -> i64 {
        assert!(idx < self.n, "Index out of bounds");
        self.tree[self.size + idx]
    }
}
