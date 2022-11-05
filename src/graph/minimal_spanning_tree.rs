use super::dgraph::Edge;
use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
pub trait MinimalSpanningTree {
    fn kruskal_minimum_spanning_tree(&self) -> Result<Vec<Edge>>;

    fn find_set(sets: &[BTreeSet<usize>], item: usize) -> Result<BTreeSet<usize>> {
        /*return the set which the item belongs to */
        if let Some(set) = sets.iter().find(|set| set.contains(&item)) {
            Ok(set.clone())
        } else {
            Err(anyhow!("Err"))
        }
    }
    fn merge_sets_and_delete_original_sets(
        sets_container: &mut Vec<BTreeSet<usize>>,
        u_set: &BTreeSet<usize>,
        v_set: &BTreeSet<usize>,
    ) -> Result<()> {
        let index = sets_container.iter().position(|set| set == u_set).unwrap();
        sets_container.remove(index);
        let index = sets_container.iter().position(|set| set == v_set).unwrap();
        sets_container.remove(index);
        let uv_set = u_set.union(v_set).copied().collect::<BTreeSet<_>>();
        sets_container.push(uv_set);
        Ok(())
    }
}
