use super::Graph;
use crate::heap::Heap;

#[derive(Copy, Clone, Debug)]
pub struct HeapElement<K, T>(K, T);

impl<K, T: PartialEq> PartialEq for HeapElement<K, T> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<K, T: Eq> Eq for HeapElement<K, T> {}
impl<K, T: PartialOrd> PartialOrd for HeapElement<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<K, T: Ord> Ord for HeapElement<K, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

pub fn djikstra(graph: &Graph<i64, u64>, start: i64) -> Vec<HeapElement<i64, u64>> {
    let mut heap = Heap::new();
    let mut answer = Vec::new();

    heap.insert(HeapElement(start, 0));

    graph.content.iter().for_each(|(vertex, _)| {
        if *vertex != start {
            heap.insert(HeapElement(*vertex, u64::MAX - 1000))
        }
    });

    loop {
        match heap.pop() {
            Some(HeapElement(subject, subject_distance)) => {
                let neighbors = graph.adjacency_list(&subject).unwrap();

                neighbors.iter().for_each(|(neighbor, neighbor_weight)| {
                    let element = heap
                        .data
                        .iter_mut()
                        .enumerate()
                        .find(|(_, HeapElement(x, _))| *neighbor == *x);

                    match element {
                        Some((index, HeapElement(_, neighbor_distance)))
                            if *neighbor_distance > neighbor_weight + subject_distance =>
                        {
                            *neighbor_distance = neighbor_weight + subject_distance;
                            heap.heapify(index);
                        }
                        _ => {}
                    }

                });

                answer.push(HeapElement(subject, subject_distance));
            }
            None => return answer,
        }
    }
}
