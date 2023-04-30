mod graph;
use graph::generate::generate_undirected;
use graph::generate::GenerationParameters;

use crate::graph::djikstra::djikstra;
fn main() {
    let parameters = GenerationParameters{vertex_count: 10, neighbor_min: 1, neighbor_max: 3};

    let graph = generate_undirected(&parameters).unwrap();

    print!("{:?}", graph);

    //graph.print_to_file("result.graph").ok();
    
    djikstra(&graph, 0, 1);
}
