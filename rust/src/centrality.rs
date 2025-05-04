use petgraph::graph::Graph;
use petgraph::Undirected;
use petgraph::algo::floyd_warshall;

/// Degree centrality: (node_index, degree)
/// Simply counts the number of neighbors for each node.
pub fn degree_centrality(
    g: &Graph<String, f64, Undirected>
) -> Vec<(usize, usize)> {
    g.node_indices()
     .map(|n| (n.index(), g.neighbors(n).count()))
     .collect()
}

/// Closeness centrality: (node_index, closeness_score)
///
/// closeness = 1.0 / sum(shortest-path distances to all other nodes)
pub fn closeness_centrality(
    g: &Graph<String, f64, Undirected>,
) -> Vec<(usize, f64)> {
    let fw = floyd_warshall(&*g, |e| *e.weight()).unwrap();

    g.node_indices()
     .map(|n| {
         let total_dist: f64 = g.node_indices()
             .filter_map(|m| fw.get(&(n, m)).cloned())
             .sum();
         let score = if total_dist > 0.0 { 1.0 / total_dist } else { 0.0 };
         (n.index(), score)
     })
     .collect()
}
