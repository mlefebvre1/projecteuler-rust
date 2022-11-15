use anyhow::Result;
use itertools::Itertools;

pub fn p() -> Result<String> {
    /*
    Magic 5-gon ring
    Problem 68
    Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding to nine.
    Working clockwise, and starting from the group of three with the numerically lowest external node (4,3,2 in this
    example), each solution can be described uniquely. For example, the above solution can be described by the set:
    4,3,2; 6,2,1; 5,1,3.
    See https://projecteuler.net/problem=68 for the diagram
    It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are eight solutions in
    total.
    Total	Solution Set
    9	4,2,3; 5,3,1; 6,1,2
    9	4,3,2; 6,2,1; 5,1,3
    10	2,3,5; 4,5,1; 6,1,3
    10	2,5,3; 6,3,1; 4,1,5
    11	1,4,6; 3,6,2; 5,2,4
    11	1,6,4; 5,4,2; 3,2,6
    12	1,5,6; 2,6,4; 3,4,5
    12	1,6,5; 3,5,4; 2,4,6
    By concatenating each group it is possible to form 9-digit strings; the maximum string for a 3-gon ring is
    432621513.
    Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings. What is
    the maximum 16-digit string for a "magic" 5-gon ring?
    */
    const INNER_GRAPH_NODE_ID: [usize; 5] = [1, 3, 5, 7, 0];
    let nodes = [1, 2, 3, 4, 5, 7, 8, 9, 10];
    let nodes_permutations = nodes.iter().permutations(9);
    let nodes_permutations_filtered = nodes_permutations.filter(|nodes| {
        for id_ in INNER_GRAPH_NODE_ID {
            if *nodes[id_] == 10 {
                return false;
            }
        }
        true
    });
    let solutions = nodes_permutations_filtered.filter_map(|nodes| {
        let gon_ring = build_magic_gon_ring(&nodes);
        if is_all_arms_sum_equal(&gon_ring) && is_first_node_the_smallest_external_node(&gon_ring) {
            return Some(build_string_from_gon_ring(&gon_ring));
        }
        None
    });
    let solutions_as_int = solutions.map(|solution_as_vec_u8| {
        let mut s = String::new();
        for c in solution_as_vec_u8 {
            s += &c.to_string();
        }
        s.parse::<usize>().unwrap()
    });
    Ok(solutions_as_int.max().unwrap().to_string())
}

fn is_all_arms_sum_equal(gon_ring: &[(u8, u8, u8)]) -> bool {
    let mut arm_sums = gon_ring.iter().map(|&arm| arm.0 + arm.1 + arm.2);
    let arm_sum_to_match = arm_sums.next().unwrap();
    for arm_sum in arm_sums {
        if arm_sum != arm_sum_to_match {
            return false;
        }
    }
    true
}

fn is_first_node_the_smallest_external_node(gon_ring: &[(u8, u8, u8)]) -> bool {
    let external_nodes = gon_ring.iter().map(|&arm| arm.0).collect::<Vec<u8>>();
    let min_external_node = external_nodes.iter().min().unwrap();
    external_nodes[0] == *min_external_node
}

fn build_magic_gon_ring(nodes: &[&u8]) -> Vec<(u8, u8, u8)> {
    vec![
        (6, *nodes[0], *nodes[1]),
        (*nodes[2], *nodes[1], *nodes[3]),
        (*nodes[4], *nodes[3], *nodes[5]),
        (*nodes[6], *nodes[5], *nodes[7]),
        (*nodes[8], *nodes[7], *nodes[0]),
    ]
}

fn build_string_from_gon_ring(gon_ring: &[(u8, u8, u8)]) -> Vec<u8> {
    let mut result = Vec::new();
    for arm in gon_ring {
        result.push(arm.0);
        result.push(arm.1);
        result.push(arm.2);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "6531031914842725");
    }
}
