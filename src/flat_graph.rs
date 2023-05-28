use rkyv::{Archive, Deserialize, Serialize};

use crate::{graph_node::GraphNode, GraphEdge};

#[derive(Debug, Archive, Deserialize, Serialize)]
pub struct FlatGraph<D, E> {
    pub nodes: Vec<GraphNode<D, E>>,
}

impl<D, E> FlatGraph<D, E>
where
    D: Clone,
    E: Clone,
{
    pub fn new(root_data: &D) -> Self {
        Self {
            nodes: vec![GraphNode::new(root_data.clone())],
        }
    }

    pub fn append(&mut self, prev: usize, edge_data: &E, node_data: &D) {
        let node = GraphNode::new(node_data.clone());
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        self.nodes[prev]
            .edges
            .push(GraphEdge::new(edge_data.clone(), index));
    }
}

#[cfg(test)]
mod flat_graph_tests {
    use super::*;

    #[test]
    fn create_root() {
        let root_data = "Hello, world!";
        let graph: FlatGraph<&str, &str> = FlatGraph::new(&root_data);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].data, root_data);
        println!("{:?}", graph);
    }

    #[test]
    fn append_child() {
        let root_data = "Hello, world!";
        let mut graph: FlatGraph<&str, &str> = FlatGraph::new(&root_data);
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
