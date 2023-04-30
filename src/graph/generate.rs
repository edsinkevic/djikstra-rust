use super::Graph;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum GraphGenerationError {
    InvalidNeighborMin,
    InvalidNeighborMax,
    TooManyEdges,
}

pub struct GenerationParameters {
    pub vertex_count: i64,
    pub neighbor_min: i64,
    pub neighbor_max: i64,
}

pub fn generate_undirected(
    parameters: &GenerationParameters,
) -> Result<Graph<i64, u64>, GraphGenerationError> {
    args_validation(parameters)?;

    let mut graph = Graph::new();

    let vertices = 0..parameters.vertex_count;

    let mut rng = rand::thread_rng();

    vertices.for_each(|vertex| fill_vertex(&mut graph, vertex, &mut rng, &parameters));

    return Ok(graph);
}

fn fill_vertex(
    graph: &mut Graph<i64, u64>,
    vertex: i64,
    rand: &mut ThreadRng,
    parameters: &GenerationParameters,
) -> () {
    if !graph.vertex_exists(&vertex) {
        graph.insert_vertex(vertex);
    }

    let neighbors_count = graph.out_neighbors(&vertex).unwrap().len() as i64;

    let random = rand.gen_range(parameters.neighbor_min..parameters.neighbor_max);
    let random_neighbor_count = (random - neighbors_count).max(0);

    (0..random_neighbor_count).for_each(|_| {
        let mut random_neighbor;

        loop {
            random_neighbor = rand.gen_range(0..parameters.vertex_count);
            random_neighbor = make_sure_not_same(vertex, random_neighbor, parameters.vertex_count);
            if !graph.vertex_exists(&random_neighbor) {
                graph.insert_vertex(random_neighbor);
            }

            let neighbor_has_space_for_neighbors =
                (graph.adjacency_list(&random_neighbor).unwrap().len() as i64 + 1)
                    < parameters.neighbor_max;

            if neighbor_has_space_for_neighbors && !graph.edge_exists(&vertex, &random_neighbor) {
                break;
            }
        }

        let rand_value = rand.gen_range(0..100);
        graph.insert_edge(vertex, random_neighbor, rand_value);
        graph.insert_edge(random_neighbor, vertex, rand_value);
    });
}

fn make_sure_not_same(a: i64, vert: i64, max: i64) -> i64 {
    let mut vertex = vert;
    if a == vertex {
        vertex = vertex + 1;
    }

    if max <= vertex {
        vertex = vertex - 2;
    }

    vertex
}

fn args_validation(args: &GenerationParameters) -> Result<(), GraphGenerationError> {
    if args.neighbor_max >= args.vertex_count {
        return Err(GraphGenerationError::InvalidNeighborMax);
    }

    if args.neighbor_min >= args.vertex_count {
        return Err(GraphGenerationError::InvalidNeighborMin);
    }

    let max_edge_count = (args.vertex_count * (args.vertex_count - 1)) / 2;

    if max_edge_count < args.neighbor_min * args.vertex_count {
        return Err(GraphGenerationError::TooManyEdges);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::graph::generate::make_sure_not_same;

    use super::{generate_undirected, GenerationParameters, GraphGenerationError};

    #[test]
    fn args_validation() {
        let parameters = GenerationParameters {
            vertex_count: 10,
            neighbor_min: 0,
            neighbor_max: 3,
        };
        let graph = generate_undirected(&parameters);
        assert!(graph.is_ok());

        let parameters = GenerationParameters {
            vertex_count: 5,
            neighbor_min: 0,
            neighbor_max: 6,
        };
        let err = generate_undirected(&parameters);
        assert_eq!(err.unwrap_err(), GraphGenerationError::InvalidNeighborMax);

        let parameters = GenerationParameters {
            vertex_count: 5,
            neighbor_min: 6,
            neighbor_max: 3,
        };
        let err = generate_undirected(&parameters);
        assert_eq!(err.unwrap_err(), GraphGenerationError::InvalidNeighborMin);

        let parameters = GenerationParameters {
            vertex_count: 5,
            neighbor_min: 3,
            neighbor_max: 3,
        };
        let err = generate_undirected(&parameters);
        assert_eq!(err.unwrap_err(), GraphGenerationError::TooManyEdges);
    }

    #[test]
    fn make_sure_not_same_1() {
        let vert = 10;

        let result = make_sure_not_same(10, vert, 10);
        assert_eq!(result, 9);

        let result = make_sure_not_same(10, vert, 11);
        assert_eq!(result, 9);

        let result = make_sure_not_same(10, vert, 12);
        assert_eq!(result, 11);
    }

    #[test]
    fn generate_undirected_1() {
        let parameters = GenerationParameters {
            vertex_count: 10,
            neighbor_min: 0,
            neighbor_max: 3,
        };

        let graph = generate_undirected(&parameters).unwrap();

        assert_eq!(graph.vertex_count() as i64, parameters.vertex_count);
        graph.content.iter().for_each(|(_, edge_list)| {
            let len = edge_list.len() as i64;
            assert!(len < parameters.neighbor_max);
            assert!(len >= parameters.neighbor_min);
        })
    }
}
