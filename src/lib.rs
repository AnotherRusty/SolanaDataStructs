pub mod flat_graph;
pub mod flat_graph_single;
pub mod flat_graph_visitor;
pub mod flat_graph_visitor_single;
pub mod graph_node;
pub mod graph_node_single;

pub use flat_graph::*;
pub use flat_graph_single::*;
pub use flat_graph_visitor::*;
pub use flat_graph_visitor_single::*;
pub use graph_node::*;
pub use graph_node_single::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
