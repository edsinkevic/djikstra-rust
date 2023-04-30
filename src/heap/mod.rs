pub struct Heap<T: Ord + Copy> {
    pub data: Vec<T>,
}

impl<T: Ord + Copy> Heap<T> {
    pub fn new() -> Self {
        Heap { data: vec![] }
    }

    pub fn insert(&mut self, element: T) {
        self.data.push(element);
        let mut last_element_idx = self.data.len() - 1;

        while last_element_idx != 0 {
            let parent_idx = self.parent(last_element_idx);

            if self.data[last_element_idx] < self.data[parent_idx] {
                self.data.swap(last_element_idx, parent_idx)
            }

            last_element_idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.len() {
            n if n == 0 => None,
            n if n == 1 => self.data.pop(),
            _ => {
                let last_element_idx = self.data.len() - 1;
                self.data.swap(0, last_element_idx);
                let value = self.data.pop().unwrap();

                self.heapify(0);

                Some(value)
            }
        }
    }

    fn parent(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    pub fn heapify(&mut self, idx: usize) {
        let mut current = idx;

        loop {
            let left = self.left(current);
            let right = self.right(current);
            let size = self.data.len() - 1;

            if left > size || right > size {
                break;
            }

            let max = self.get_max_idx(left, right, current);

            if max != current {
                self.data.swap(current, max);

                current = max;
            } else {
                break;
            }
        }
    }

    fn get_max_idx(&self, left_idx: usize, right_idx: usize, idx: usize) -> usize {
        let max = self.data[left_idx]
            .max(self.data[right_idx])
            .max(self.data[idx]);

        match max {
            i if i == self.data[idx] => idx,
            i if i == self.data[right_idx] => right_idx,
            _ => left_idx,
        }
    }
}

#[cfg(test)]
mod tests {}
