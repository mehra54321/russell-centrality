mod graph;
mod centrality;

use std::fs::File;
use csv::Writer;
//use petgraph::graph::NodeIndex;

fn main() {
    // 1) Load the graph
    let (graph, idx_map) = graph::load_graph("/Users/atharvamehra/Documents/nasdaq-centrality/data/russ_edges.csv");

    // 2) Compute centrality measures
    let degrees     = centrality::degree_centrality(&graph);
    let closenesses = centrality::closeness_centrality(&graph);

    // 3) Prepare CSV writer
    let mut wtr = Writer::from_writer(
        File::create("../data/centrality_scores.csv").unwrap()
    );
    wtr.write_record(&["Ticker", "Degree", "Closeness"]).unwrap();

    // 4) Emit each node’s scores
    for (node_i, deg) in degrees {
        // a) find the ticker string for this node index
        let ticker = idx_map.iter()
            .find(|pair| pair.1.index() == node_i)
            .map(|(t, _)| t.clone())
            .unwrap();

        // b) find its closeness score
        let clo = closenesses.iter()
            .find(|pair| pair.0 == node_i)
            .map(|pair| pair.1)
            .unwrap();

        wtr.write_record(&[
            ticker,
            deg.to_string(),
            clo.to_string(),
        ]).unwrap();
    }

    wtr.flush().unwrap();
    println!("→ data/centrality_scores.csv written");
}
