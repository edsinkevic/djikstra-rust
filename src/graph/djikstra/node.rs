#[derive(Copy, Clone, Debug)]
pub struct Node<K, T> {
    pub vertex: K,
    pub distance: T,
}

impl<K, T: PartialEq> PartialEq for Node<K, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<K, T: Eq> Eq for Node<K, T> {}
impl<K, T: PartialOrd> PartialOrd for Node<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<K, T: Ord> Ord for Node<K, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}
