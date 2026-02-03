pub struct FenwickTree {
    size: usize,
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        FenwickTree {
            size,
            tree: vec![0; size + 1],
        }
    }

    pub fn from_vec(arr: &[i64]) -> Self {
        let mut ft = Self::new(arr.len());
        for (i, &val) in arr.iter().enumerate() {
            ft.add(i, val);
        }
        ft
    }

    pub fn add(&mut self, index: usize, value: i64) {
        assert!(index < self.size, "Index out of bounds");

        let mut idx = index + 1;
        while idx <= self.size {
            self.tree[idx] += value;
            idx += idx & (!idx + 1);
        }
    }

    pub fn sum(&self, index: usize) -> i64 {
        if index >= self.size {
            return self.sum(self.size - 1);
        }

        let mut sum = 0;
        let mut idx = index + 1;
        while idx > 0 {
            sum += self.tree[idx];
            idx &= idx - 1;
        }
        sum
    }

    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        assert!(left <= right && right < self.size, "Invalid range");

        if left == 0 {
            self.sum(right)
        } else {
            self.sum(right) - self.sum(left - 1)
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        self.range_sum(index, index)
    }

    pub fn set(&mut self, index: usize, value: i64) {
        let current = self.get(index);
        self.add(index, value - current);
    }
}
