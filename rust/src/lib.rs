// src/lib.rs

pub mod graph;
pub mod centrality;

#[cfg(test)]
mod tests {
    use tempfile::tempdir;  // dev‐dependency, only in tests
    use super::graph::load_graph;
    use super::centrality::{degree_centrality, closeness_centrality};
    use petgraph::graph::Graph;
    use petgraph::Undirected;

    #[test]
    fn test_degree_on_small_graph() {
        // Triangle A—B—C—A
        let mut g = Graph::<String, f64, Undirected>::new_undirected();
        let a = g.add_node("A".into());
        let b = g.add_node("B".into());
        let c = g.add_node("C".into());
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g.add_edge(a, c, 1.0);

        // degree_centrality returns Vec<(node_index, degree)>
        let deg = degree_centrality(&g);
        // extract just the degree values
        let degrees: Vec<usize> = deg.into_iter().map(|(_idx, d)| d).collect();
        // All degrees should be 2
        assert_eq!(degrees, vec![2, 2, 2]);
    }

    #[test]
    fn test_closeness_on_line_graph() {
        // Line A—B—C
        let mut g = Graph::<String, f64, Undirected>::new_undirected();
        let a = g.add_node("A".into());
        let b = g.add_node("B".into());
        let c = g.add_node("C".into());
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);

        let clo = closeness_centrality(&g);
        // B should have higher closeness than A
        assert!(clo[b.index()] > clo[a.index()]);
    }

    #[test]
    fn test_load_graph_from_csv() {
        use std::fs::File;
        use std::io::Write;

        // Create a temporary directory and CSV file
        let dir = tempdir().unwrap();
        let path = dir.path().join("edges.csv");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "Source,Target,Weight").unwrap();
        writeln!(file, "X,Y,0.8").unwrap();
        writeln!(file, "Y,Z,0.7").unwrap();

        let (g, idx) = load_graph(path.to_str().unwrap());
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert!(idx.contains_key("X"));
        assert!(idx.contains_key("Y"));
        assert!(idx.contains_key("Z"));
    }
}
