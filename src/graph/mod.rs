pub mod djikstra;
pub mod generate;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::{Debug, Display},
    hash,
    io::Write,
};

use std::fs::File;

#[derive(Debug)]
pub struct Graph<K, T> {
    content: HashMap<K, LinkedList<(K, T)>>,
}

impl<K, T> Graph<K, T>
where
    K: hash::Hash + Eq + Copy + Display + Debug,
    T: Debug + Display,
{
    pub fn new() -> Graph<K, T> {
        Graph {
            content: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn insert_vertex(&mut self, vertex: K) -> Option<LinkedList<(K, T)>> {
        self.content.insert(vertex, LinkedList::new())
    }

    pub fn vertex_exists(&self, vertex: &K) -> bool {
        self.content.contains_key(vertex)
    }

    pub fn vertex_count(&self) -> usize {
        self.content.len()
    }

    pub fn adjacency_list(&self, vertex: &K) -> Option<&LinkedList<(K, T)>> {
        self.content.get(vertex)
    }

    fn adjacency_list_mut(&mut self, vertex: &K) -> Option<&mut LinkedList<(K, T)>> {
        self.content.get_mut(vertex)
    }

    pub fn insert_edge(&mut self, from: K, to: K, value: T) -> Option<()> {
        (self.vertex_exists(&from) && self.vertex_exists(&to)).then(|| {
            self.adjacency_list_mut(&from)
                .unwrap()
                .push_front((to, value))
        })
    }

    pub fn edge_exists(&self, from: &K, to: &K) -> bool {
        self.vertex_exists(from)
            && self
                .adjacency_list(from)
                .unwrap()
                .iter()
                .any(|(neighbor, _)| *neighbor == *to)
    }

    pub fn out_neighbors(&self, vertex: &K) -> Option<HashSet<&K>> {
        Some(
            self.adjacency_list(&vertex)?
                .iter()
                .map(|(out_neighbor, _)| out_neighbor)
                .collect(),
        )
    }

    pub fn in_neighbors(&self, vertex: &K) -> Option<HashSet<&K>> {
        if !self.vertex_exists(vertex) {
            return None;
        };

        Some(
            self.content
                .keys()
                .filter_map(|key| {
                    if key != vertex && self.edge_exists(key, vertex) {
                        Some(key)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    pub fn print_to_file(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        self.content.iter().for_each(|(vertex, edge_list)| {
            write!(&mut file, "Vertex {}: {:?}\n", vertex, edge_list).ok();
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::graph::Graph;

    #[test]
    fn is_empty() {
        let graph = Graph::<i64, i64>::new();
        assert_eq!(graph.is_empty(), true);
    }

    #[test]
    fn insert() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        assert_eq!(graph.insert_vertex(vertex), None);
        assert_eq!(graph.is_empty(), false);
    }

    #[test]
    fn insert_edge_when_vertex_does_not_exist() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let to = 12;
        graph.insert_edge(vertex, to, 0);
        assert_eq!(graph.is_empty(), true);
    }

    #[test]
    fn vertex_exists() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        graph.insert_vertex(vertex);
        assert!(graph.content.contains_key(&vertex));
        assert!(graph.vertex_exists(&vertex))
    }

    #[test]
    fn adjacency_list() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let to = 12;
        graph.insert_vertex(vertex);
        graph.insert_vertex(to);
        graph.insert_edge(vertex, to, 0);
        let list = graph.adjacency_list(&vertex).unwrap();

        assert_eq!(*list, LinkedList::from([(to, 0)]));
    }

    #[test]
    fn insert_edge() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let to = 12;
        graph.insert_vertex(vertex);
        graph.insert_vertex(to);
        assert!(graph.insert_edge(vertex, to, 0).is_some());
        assert!(!graph.adjacency_list(&vertex).unwrap().is_empty());
    }

    #[test]
    fn edge_exists() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let to = 12;
        graph.insert_vertex(vertex);
        graph.insert_vertex(to);
        assert!(graph.content.contains_key(&vertex));
        let result = graph.insert_edge(vertex, to, 0);
        assert!(result.is_some());

        assert_eq!(graph.edge_exists(&vertex, &to), true);
        assert_eq!(graph.edge_exists(&to, &vertex), false);
    }

    #[test]
    fn out_neighbors() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let vertex1 = 12;
        let vertex2 = 13;
        graph.insert_vertex(vertex);
        graph.insert_vertex(vertex1);
        graph.insert_vertex(vertex2);
        graph.insert_edge(vertex, vertex1, 0);
        graph.insert_edge(vertex, vertex2, 1);
        graph.insert_edge(vertex, vertex2, 1);
        graph.insert_edge(vertex2, vertex, 1);
        assert_eq!(graph.out_neighbors(&vertex).unwrap().len(), 2);
    }

    #[test]
    fn in_neighbors() {
        let mut graph = Graph::<i64, i64>::new();
        let vertex = 10;
        let vertex1 = 12;
        let vertex2 = 13;
        graph.insert_vertex(vertex);
        graph.insert_vertex(vertex1);
        graph.insert_vertex(vertex2);
        graph.insert_edge(vertex1, vertex, 0);
        graph.insert_edge(vertex2, vertex, 1);
        assert_eq!(graph.in_neighbors(&vertex).unwrap().len(), 2);
    }
}
