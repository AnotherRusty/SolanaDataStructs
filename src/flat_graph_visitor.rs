use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{Archive, Deserialize, Serialize};

use crate::FlatGraph;

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct FlatGraphVisitor<'a, D, E> {
    pub graph: &'a FlatGraph<D, E>,
    pub index: usize,
    pub path: Vec<usize>,
}

impl<'a, D, E> FlatGraphVisitor<'a, D, E> {
    pub fn new(graph: &'a FlatGraph<D, E>) -> Self {
        Self {
            graph,
            index: 0,
            path: Vec::new(),
        }
    }

    pub fn advance(&mut self, edge: usize) {
        self.path.push(self.index);
        self.index = self.graph.nodes[self.index].edges[edge].node;
    }
}

#[cfg(test)]
mod flat_graph_visitor_tests {
    use super::*;

    #[test]
    fn create() {
        let root_data = "Hello, world!";
        let graph: FlatGraph<&str, &str> = FlatGraph::new(&root_data);
        let visitor: FlatGraphVisitor<&str, &str> = FlatGraphVisitor::new(&graph);
        assert_eq!(visitor.index, 0);
        assert_eq!(visitor.path.len(), 0);
        println!("{:?}", visitor);
    }

    #[test]
    fn advance() {
        let root_data = "Hello, world!";
        let mut graph: FlatGraph<&str, &str> = FlatGraph::new(&root_data);
        let child_data = "Hello, child!";
        graph.append(0, &"to child", &child_data);
        let mut visitor = FlatGraphVisitor::new(&graph);
        assert_eq!(visitor.index, 0);
        assert_eq!(visitor.path.len(), 0);
        println!("{:?}", visitor);
        visitor.advance(0);
        assert_eq!(visitor.index, 1);
        assert_eq!(visitor.path.len(), 1);
        assert_eq!(visitor.path[0], 0);
        println!("{:?}", visitor);
    }
}
