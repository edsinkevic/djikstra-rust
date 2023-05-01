pub struct Heap<T: Ord + Copy> {
    pub data: Vec<T>,
}

impl<T: Ord + Copy> Heap<T> {
    pub fn new() -> Self {
        Heap { data: vec![] }
    }

    pub fn insert(&mut self, element: T) {
        self.data.push(element);
        let mut current = self.data.len() - 1;

        while current != 0 {
            let parent_idx = self.parent(current);

            if self.data[current] < self.data[parent_idx] {
                self.data.swap(current, parent_idx)
            }

            current = parent_idx;
        }
    }

    pub fn decrease_key(&mut self, idx: usize, element: T) {
        if element > self.data[idx] {
            panic!("New key is bigger than current key");
        }

        self.data[idx] = element;
        let mut current = idx;

        while current > 0 && self.data[self.parent(current)] > self.data[current] {
            let parent = self.parent(current);
            self.data.swap(current, parent);
            current = self.parent(current);
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
        if idx % 2 == 0 {
            (idx - 1) / 2
        } else {
            idx / 2
        }
    }

    fn left(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    pub fn heapify(&mut self, idx: usize) {
        let left_idx = self.left(idx);
        let right_idx = self.right(idx);
        let size = self.data.len();
        let mut min_idx = idx;

        if left_idx < size && self.data[left_idx] < self.data[min_idx] {
            min_idx = left_idx;
        };

        if right_idx < size && self.data[right_idx] < self.data[min_idx] {
            min_idx = right_idx;
        }

        if min_idx != idx {
            self.data.swap(idx, min_idx);
            self.heapify(min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn insert() {
        let mut heap = Heap::new();
        heap.insert(3);
        heap.insert(2);
        heap.insert(1);

        assert_eq!(heap.pop().unwrap(), 1);
        assert_eq!(heap.pop().unwrap(), 2);
        assert_eq!(heap.pop().unwrap(), 3);
    }

    #[test]
    fn parent() {
        let heap = Heap::<i64>::new();
        assert_eq!(heap.right(1), 4);
        assert_eq!(heap.parent(4), 1);
    }
}
