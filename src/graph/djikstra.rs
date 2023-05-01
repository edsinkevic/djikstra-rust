use super::Graph;
use crate::heap::Heap;

#[derive(Copy, Clone, Debug)]
pub struct Node<K, T>{
    vertex: K,
    distance: T
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

pub fn djikstra(graph: &Graph<i64, u64>, start: i64) -> Vec<Node<i64, u64>> {
    let mut heap = Heap::new();
    let mut answer = Vec::new();

    if !graph.vertex_exists(&start) {
        return Vec::new();
    }

    heap.insert(Node{vertex: start, distance: 0});

    graph.content.iter().for_each(|(vertex, _)| {
        if *vertex != start {
            heap.insert(Node{vertex: *vertex, distance: u64::MAX - 1000})
        }
    });

    loop {
        match heap.pop() {
            Some(subject) => {
                let neighbors = graph.adjacency_list(&subject.vertex).unwrap();

                neighbors.iter().for_each(|(neighbor, neighbor_weight)| {
                    let element = heap
                        .data
                        .iter_mut()
                        .enumerate()
                        .find(|(_, adjacent)| *neighbor == adjacent.vertex);

                    match element {
                        Some((index, neighbor_node))
                            if neighbor_node.distance > neighbor_weight + subject.distance =>
                        {
                            neighbor_node.distance = neighbor_weight + subject.distance;
                            println!("Setting {} (heap.data index {}) to {}", neighbor_node.vertex, index, neighbor_weight + subject.distance);
                            println!("Before heapify: {:?}", heap.data);
                            heap.heapify(index);
                            println!("After heapify: {:?}", heap.data);
                        }
                        _ => {}
                    }
                });

                answer.push(subject);
            }
            None => break,
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use crate::graph::{Graph, djikstra::{djikstra, Node}};

    #[test]
    fn empty() {
        let graph = Graph::<i64, u64>::new();
        assert_eq!(djikstra(&graph, 0), Vec::new());
    }

    #[test]
    fn test1() {
        let mut graph = Graph::new();

        graph.insert_vertex(0);
        graph.insert_vertex(1);
        graph.insert_vertex(2);
    
        graph.insert_edge(0, 1, 50);
        graph.insert_edge(1, 0, 50);
        graph.insert_edge(0, 2, 30);
        graph.insert_edge(2, 0, 30);
        graph.insert_edge(2, 1, 10);
        graph.insert_edge(1, 2, 10);
        
        let results = djikstra(&graph, 0);
        println!("{:?}", results);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Node{vertex: 0, distance: 0});
        assert_eq!(results[1], Node{vertex: 2, distance: 30});
        assert_eq!(results[2], Node{vertex: 1, distance: 40});
    }
}