use crate::graph::{
    dgraph::{BuildGraph, Dgraph, Edge},
    shortest_path::ShortestPath,
};
use crate::utils::matrix::load_matrix2d_from_file;
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Path sum: four ways
    Problem 83

    In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right,
    by moving left, right, up, and down, is indicated in bold red and is equal to 2297.

    Find the minimal path sum from the top left to the bottom right by moving left, right, up,
    and down in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing
    an 80 by 80 matrix.

    Couldn't come up with a better algorithm than dijkstra for solving this one.. It's still quick.. under 1 second
    */
    let mut matrix_graph = MatrixGraph::new("src/problems/data/problem83.txt");
    matrix_graph.build_graph();
    let shortest_dist = matrix_graph
        .graph
        .disjktra_shortest_path(0, matrix_graph.graph.vertices.len() - 1)
        + matrix_graph.matrix[[0, 0]];
    Ok(shortest_dist.to_string())
}

struct MatrixGraph {
    graph: Dgraph,
    matrix: ndarray::Array2<isize>,
}

impl MatrixGraph {
    pub fn new(file: &str) -> Self {
        let matrix = load_matrix2d_from_file::<isize>(file, ',');
        let shape = matrix.shape();
        let [y_len, x_len] = [shape[0], shape[1]];
        Self {
            graph: Dgraph::new(x_len * y_len),
            matrix,
        }
    }

    fn append_edges(&mut self, vertex_id: usize, right: bool, left: bool, down: bool, up: bool) {
        let shape = self.matrix.shape();
        let [_, x_len] = [shape[0], shape[1]];
        let (x, y) = (vertex_id % x_len, vertex_id / x_len);
        if right {
            self.graph
                .add_edge(Edge::new(vertex_id, vertex_id + 1, self.matrix[[y, x + 1]]));
        }
        if left {
            self.graph
                .add_edge(Edge::new(vertex_id, vertex_id - 1, self.matrix[[y, x - 1]]));
        }
        if down {
            self.graph.add_edge(Edge::new(
                vertex_id,
                vertex_id + x_len,
                self.matrix[[y + 1, x]],
            ));
        }
        if up {
            self.graph.add_edge(Edge::new(
                vertex_id,
                vertex_id - x_len,
                self.matrix[[y - 1, x]],
            ));
        }
    }
}
impl BuildGraph for MatrixGraph {
    fn build_graph(&mut self) {
        let shape = self.matrix.shape();
        let [y_len, x_len] = [shape[0], shape[1]];
        for vertex_id in 0..self.graph.vertices.len() {
            let (x, y) = (vertex_id % x_len, vertex_id / x_len);
            if y == 0 {
                if x == 0 {
                    // Right and Down
                    self.append_edges(vertex_id, true, false, true, false);
                } else if x == x_len - 1 {
                    // Left and Down
                    self.append_edges(vertex_id, false, true, true, false);
                } else {
                    // Right, Left and Down
                    self.append_edges(vertex_id, true, true, true, false);
                }
            } else if y == y_len - 1 {
                if x == 0 {
                    // Right and Up
                    self.append_edges(vertex_id, true, false, false, true);
                } else if x == x_len - 1 {
                    // Nothing..
                    continue;
                } else {
                    // Right, Left, Up
                    self.append_edges(vertex_id, true, true, false, true);
                }
            } else if x == 0 {
                // Right, Down, Up
                self.append_edges(vertex_id, true, false, true, true);
            } else if x >= x_len - 1 {
                // Left, Down and Up
                self.append_edges(vertex_id, false, true, true, true);
            } else {
                // Right, Left, Down, Up
                self.append_edges(vertex_id, true, true, true, true);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "425185");
    }
}
