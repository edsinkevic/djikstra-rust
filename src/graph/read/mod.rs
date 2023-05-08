use std::{
    collections::{HashMap, LinkedList},
    fs::read_to_string,
};

use super::Graph;

pub fn read_from_file(file_name: &str) -> std::io::Result<Graph<i64, u64>> {
    let file_contents = read_to_string(file_name)?;
    let mut graph_content = HashMap::new();

    for (idx, line) in file_contents.lines().enumerate() {
        let mut main_cursor = find_next_int(line).expect(&format!("Start required! Line {}", idx));
        let mut edge_list = LinkedList::new();
        let vertex = parse_next_int(main_cursor);
        loop {
            match find_next_int(main_cursor) {
                Some(cursor) => {
                    let edge_vertex = parse_next_int(cursor);
                    let cursor = find_next_int(cursor).expect(&format!("Weight required! {}", idx));
                    let weight = parse_next_int(cursor) as u64;
                    edge_list.push_front((edge_vertex, weight));
                    main_cursor = cursor;
                }
                None => break,
            }
        }
        graph_content.insert(
            vertex,
            LinkedList::from_iter(edge_list.iter().rev().cloned()),
        );
    }

    Ok(Graph {
        content: graph_content,
    })
}

fn parse_next_int(line: &str) -> i64 {
    line.chars()
        .take_while(|ch| ch.is_digit(10))
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
        })
        .expect("OMG")
        .into()
}

fn find_next_int(line: &str) -> Option<&str> {
    line.char_indices()
        .skip_while(|(_, ch)| ch.is_digit(10))
        .skip_while(|(_, ch)| !ch.is_digit(10))
        .nth(0)
        .map(|(idx, _)| &line[idx..])
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::{print::print_to_file, Graph};

    use super::read_from_file;

    #[test]
    fn read_from_file_() {
        let mut graph = Graph::new();
        graph.insert_vertex(0);
        graph.insert_vertex(1);
        graph.insert_vertex(2);

        graph.insert_edge(0, 1, 20);
        graph.insert_edge(0, 2, 40);
        graph.insert_edge(1, 2, 60);
        let file_name = "files/test.graph";
        print_to_file(&graph, file_name).expect("Couldn't print graph!");
        let same_graph = read_from_file(file_name).expect("Couldn't read graph!");
        assert_eq!(
            HashSet::<i64>::from_iter(graph.get_vertices().iter().cloned().cloned()),
            HashSet::<i64>::from_iter(same_graph.get_vertices().iter().cloned().cloned())
        );

        assert_eq!(graph.content, same_graph.content);
    }
}
