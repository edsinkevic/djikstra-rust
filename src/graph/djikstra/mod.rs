pub mod node;

use self::node::Node;

use super::Graph;
use crate::heap::Heap;

pub fn djikstra(graph: &Graph<i64, u64>, start: i64) -> Vec<Node<i64, u64>> {
    let mut heap = Heap::new();
    let mut answer = Vec::new();

    init(&mut heap, graph, start);

    loop {
        let popped = heap.pop();

        if popped.is_none() {
            break;
        }

        let subject = popped.unwrap();

        let neighbors = graph.adjacency_list(&subject.vertex).unwrap();

        neighbors.iter().for_each(|(neighbor, neighbor_weight)| {
            let element = heap
                .data
                .iter()
                .enumerate()
                .find(|(_, adjacent)| *neighbor == adjacent.vertex);

            match element {
                Some((index, neighbor_node))
                    if neighbor_node.distance > neighbor_weight + subject.distance =>
                {
                    heap.decrease_key(
                        index,
                        Node {
                            vertex: neighbor_node.vertex,
                            distance: neighbor_weight + subject.distance,
                            prev: Some(subject.vertex),
                        },
                    );
                }
                _ => {}
            }
        });

        // println!("\n\n\n");
        answer.push(subject);
    }
    answer
}

fn init(heap: &mut Heap<Node<i64, u64>>, graph: &Graph<i64, u64>, start: i64) {
    if !graph.vertex_exists(&start) {
        return;
    }

    heap.insert(Node {
        vertex: start,
        distance: 0,
        prev: None,
    });

    graph.content.iter().for_each(|(vertex, _)| {
        if *vertex != start {
            heap.insert(Node {
                vertex: *vertex,
                distance: u64::MAX - 1000,
                prev: None,
            })
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::graph::{
        djikstra::{djikstra, Node},
        Graph,
    };

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
        graph.insert_vertex(3);

        graph.insert_edge(0, 1, 50);
        graph.insert_edge(1, 0, 50);
        graph.insert_edge(0, 2, 30);
        graph.insert_edge(2, 0, 30);
        graph.insert_edge(2, 1, 10);
        graph.insert_edge(1, 2, 10);
        graph.insert_edge(3, 1, 60);
        graph.insert_edge(1, 3, 60);

        let results = djikstra(&graph, 0);
        println!("{:?}", results);
        assert_eq!(results.len(), 4);
        assert_eq!(
            results,
            vec![
                Node {
                    vertex: 0,
                    distance: 0,
                    prev: None
                },
                Node {
                    vertex: 2,
                    distance: 30,
                    prev: Some(0)
                },
                Node {
                    vertex: 1,
                    distance: 40,
                    prev: Some(2)
                },
                Node {
                    vertex: 3,
                    distance: 100,
                    prev: Some(1)
                }
            ]
        );
    }
}
