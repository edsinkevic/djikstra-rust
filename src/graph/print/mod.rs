use std::{collections::LinkedList, fs::File};
use std::io::Write;

use super::Graph;

pub fn print_to_file(graph: &Graph<i64, u64>, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    let mut sorted = Vec::from_iter(graph.content.iter());

    sorted.sort_by_key(|(x, _)| **x);

    sorted.iter().for_each(|(vertex, edge_list)| {
        let vertex_str = vertex_to_string(**vertex);
        let list_str = edge_list_to_string(*edge_list);
        write!(&mut file, "{} {}\n", vertex_str, list_str).expect("Couldn't write!");
    });
    Ok(())
}

fn vertex_to_string(vert: i64) -> String {
    format!("Vertex {}: ", vert)
}

fn edge_list_to_string(list: &LinkedList<(i64, u64)>) -> String {
    let mut output = "".to_owned();
    list.iter().for_each(|(vert, weight)| {
        output.push_str(&format!("{} {} -> ", *vert, *weight));
    });

    output
}
