pub mod graph;
pub mod heap;
use graph::generate::generate_undirected;
use graph::generate::GenerationParameters;

use crate::graph::djikstra::djikstra;
use crate::graph::print::print_to_file;
use crate::graph::read::read_from_file;
use crate::graph::Graph;
fn main() {
    let parameters = GenerationParameters {
        vertex_count: 1000,
        neighbor_min: 40,
        neighbor_max: 50,
    };

    print!("Generating graph!");
    let graph = generate_undirected(&parameters).unwrap();

    // let graph= read_from_file("files/main.graph").expect("File not found!");

    print!("Printing graph!");
    print_to_file(&graph, "files/big.graph").expect("Couldn't write file!");

    let mut min = None;
    let mut min_vertex = None;

    graph.get_vertices().iter().for_each(|vertex| {
        println!("Calculating for vertex {}!", **vertex);

        let result = djikstra(&graph, **vertex);
        let mut sum = 0;
        let mut len = 0;
        result.iter().for_each(|node| {
            if node.prev.is_some() {
                sum += node.distance;
                len += 1;
            }
        });
        let avg = sum as f32 / (len as f32);

        if min.is_none() || min > Some(avg) {
            min = Some(avg);
            min_vertex = Some(**vertex);
        }

        //dbg!(result, avg, **vertex, min, min_vertex);
    });

    dbg!(min, min_vertex);
}
