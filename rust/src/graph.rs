use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use csv::Reader;

/// Load an undirected, weighted graph from CSV edge list.
/// Expects CSV with columns: Source,Target,Weight
pub fn load_graph(path: &str) 
    -> (Graph<String, f64, Undirected>, HashMap<String, NodeIndex>) 
{
    let mut rdr = Reader::from_path(path).unwrap();
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut idx_map = HashMap::new();

    for result in rdr.records() {
        let rec = result.unwrap();
        let a = rec.get(0).unwrap().to_string();
        let b = rec.get(1).unwrap().to_string();
        let w: f64 = rec.get(2).unwrap().parse().unwrap();

        let ia = *idx_map.entry(a.clone())
            .or_insert_with(|| graph.add_node(a.clone()));
        let ib = *idx_map.entry(b.clone())
            .or_insert_with(|| graph.add_node(b.clone()));

        graph.add_edge(ia, ib, w);
    }

    (graph, idx_map)
}
