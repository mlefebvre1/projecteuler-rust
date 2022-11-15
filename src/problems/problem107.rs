use crate::graph::{
    dgraph::{Dgraph, Edge},
    minimal_spanning_tree::MinimalSpanningTree,
};
use anyhow::Result;
use std::fs;

fn p() -> Result<String> {
    /*
    Minimal network
    Problem 107

    The following undirected network consists of seven vertices and twelve edges with a total weight of 243.

    The same network can be represented by the matrix below.
        A	B	C	D	E	F	G
    A	-	16	12	21	-	-	-
    B	16	-	-	17	20	-	-
    C	12	-	-	28	-	31	-
    D	21	17	28	-	18	19	23
    E	-	20	-	18	-	-	11
    F	-	-	31	19	-	-	27
    G	-	-	-	23	11	27	-

    However, it is possible to optimise the network by removing some edges and still ensure that all points on the
    network remain connected. The network which achieves the maximum saving is shown below. It has a weight of 93,
    representing a saving of 243 − 93 = 150 from the original network.

    Using network.txt (right click and 'Save Link/Target As...'), a 6K text file containing a network with forty
    vertices, and given in matrix form, find the maximum saving which can be achieved by removing redundant edges
     whilst ensuring that the network remains connected.
    */
    let matrix_graph = MatrixGraph::new("src/problems/data/problem107.txt")?;
    let tree_edges = matrix_graph.graph.kruskal_minimum_spanning_tree()?;
    let minimal_cost = tree_edges
        .into_iter()
        .map(|edge| edge.weight)
        .sum::<isize>();
    let savings = matrix_graph.total_cost - minimal_cost;
    Ok(savings.to_string())
}

struct MatrixGraph {
    graph: Dgraph,
    total_cost: isize,
}

impl MatrixGraph {
    pub fn new(file: &str) -> Result<Self> {
        let file = fs::read_to_string(file).expect("Problem opening the file");
        let nb_row = file.lines().count();
        let nb_col = nb_row;
        let mut graph = Dgraph::new(nb_row * nb_col);
        let total_cost = Self::create_all_edges(&mut graph, &file)?;
        Ok(Self { graph, total_cost })
    }
    fn create_all_edges(graph: &mut Dgraph, matrix: &str) -> Result<isize> {
        let mut total_cost = 0;
        for (y, item) in matrix.lines().enumerate() {
            let weights = item.split_terminator(',');
            for (x, weight) in weights.enumerate().skip(y) {
                if weight != "-" {
                    let weight = weight.parse::<isize>()?;
                    graph.add_edge(Edge {
                        src: y,
                        dst: x,
                        weight,
                    });
                    total_cost += weight;
                }
            }
        }
        Ok(total_cost)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "259679");
    }
}
