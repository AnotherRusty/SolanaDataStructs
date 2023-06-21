use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{Archive, Deserialize, Serialize};

use crate::{GraphEdgeSingle, GraphNodeSingle};

#[derive(Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct FlatGraphSingle<D> {
    pub nodes: Vec<GraphNodeSingle<D>>,
}

impl<D> FlatGraphSingle<D>
where
    D: Clone,
{
    pub fn new(root_data: &D) -> Self {
        Self {
            nodes: vec![GraphNodeSingle::new(root_data.clone())],
        }
    }

    pub fn append(&mut self, prev: usize, edge_data: &D, node_data: &D) {
        let node = GraphNodeSingle::new(node_data.clone());
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        self.nodes[prev]
            .edges
            .push(GraphEdgeSingle::new(edge_data.clone(), index));
    }
}

#[cfg(test)]
mod flat_graph_single_tests {
    use super::*;

    #[test]
    fn create_root() {
        let root_data = "Hello, world!";
        let graph: FlatGraphSingle<&str> = FlatGraphSingle::new(&root_data);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].data, root_data);
        println!("{:?}", graph);
    }

    #[test]
    fn append_child() {
        let root_data = "Hello, world!";
        let mut graph: FlatGraphSingle<&str> = FlatGraphSingle::new(&root_data);
        let child_data = "Hello, child!";
        graph.append(0, &"to child", &child_data);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[0].edges.len(), 1);
        assert_eq!(graph.nodes[0].edges[0].data, "to child");
        assert_eq!(graph.nodes[0].edges[0].node, 1);
        assert_eq!(graph.nodes[1].data, child_data);

        let child_data = "Hello, other child!";
        graph.append(0, &"to other child", &child_data);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes[0].edges.len(), 2);
        assert_eq!(graph.nodes[0].edges[1].data, "to other child");
        assert_eq!(graph.nodes[0].edges[1].node, 2);
        assert_eq!(graph.nodes[2].data, child_data);

        println!("{:?}", graph);
    }
}
