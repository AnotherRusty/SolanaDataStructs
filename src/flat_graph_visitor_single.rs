use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{Archive, Deserialize, Serialize};

use crate::FlatGraphSingle;

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct FlatGraphVisitorSingle<'a, D> {
    pub graph: &'a FlatGraphSingle<D>,
    pub index: usize,
    pub path: Vec<usize>,
}

impl<'a, D> FlatGraphVisitorSingle<'a, D> {
    pub fn new(graph: &'a FlatGraphSingle<D>) -> Self {
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
mod flat_graph_visitor_single_tests {
    use super::*;

    #[test]
    fn create() {
        let root_data = "Hello, world!";
        let graph: FlatGraphSingle<&str> = FlatGraphSingle::new(&root_data);
        let visitor: FlatGraphVisitorSingle<&str> = FlatGraphVisitorSingle::new(&graph);
        assert_eq!(visitor.index, 0);
        assert_eq!(visitor.path.len(), 0);
        println!("{:?}", visitor);
    }

    #[test]
    fn advance() {
        let root_data = "Hello, world!";
        let mut graph: FlatGraphSingle<&str> = FlatGraphSingle::new(&root_data);
        let child_data = "Hello, child!";
        graph.append(0, &"to child", &child_data);
        let mut visitor = FlatGraphVisitorSingle::new(&graph);
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
