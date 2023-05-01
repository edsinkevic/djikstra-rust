pub mod graph;
pub mod heap;
use graph::generate::generate_undirected;
use graph::generate::GenerationParameters;

use crate::graph::djikstra::djikstra;
use crate::graph::Graph;
fn main() {
    // let parameters = GenerationParameters {
    //     vertex_count: 12,
    //     neighbor_min: 2,
    //     neighbor_max: 3,
    // };

    // let graph = generate_undirected(&parameters).unwrap();

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
    

    let mut min = None;
    let mut min_vertex = None;

    graph.get_vertices().iter().for_each(|vertex| {
        let result = djikstra(&graph, **vertex);
        let mut sum = 0;
        result.iter().for_each(|node| sum += node.distance);
        let avg = sum as f32 / (result.len() as f32);

        if min.is_none() || min > Some(avg) {
            min = Some(avg);
            min_vertex = Some(**vertex);
        }

        println!("{:?}", result);
        println!("{:?} {:?}", avg, **vertex);
        println!("{:?} {:?}", min, min_vertex);
    });

    graph.print_to_file("files/main.graph").expect("Couldn't write file!");

}
