use extendr_api::prelude::*;

/// Given a DAG return a dataframe with the leaves and parents
/// of a node vector
/// @export
#[extendr]
fn get_leaves_as_df(dag: &orbweaver_r::DirectedAcyclicGraph, nodes: orbweaver_r::RNodesIn) -> Robj {
    let mut parent = Vec::new();
    let mut leaves = Vec::new();
    for node in nodes.iter() {
        if let Ok(node_leaves) = dag.as_inner().get_leaves_under([node]) {
            if node_leaves.is_empty() {
                parent.push(Rstr::from_string(node));
                leaves.push(Rstr::from_string(node));
                continue;
            }
            let len = node_leaves.len();
            parent.extend(std::iter::repeat(Rstr::from_string(node)).take(len));
            leaves.extend(node_leaves.iter().map(Rstr::from_string));
        }
    }
    data_frame!(parent = parent, leaves = leaves)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod orbweaverRextras;
    fn get_leaves_as_df;
}
