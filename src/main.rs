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
    graph.insert_edge(1, 3, 60);
    graph.insert_edge(3, 1, 60);

    println!("{:?}", graph);

    //graph.print_to_file("result.graph").ok();

    println!("{:?}", djikstra(&graph, 0));
}
