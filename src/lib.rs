pub mod flat_graph;
pub mod flat_graph_visitor;
pub mod graph;
pub mod graph_node;

pub use flat_graph::*;
pub use flat_graph_visitor::*;
pub use graph::*;
pub use graph_node::*;

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
