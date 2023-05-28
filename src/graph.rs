pub trait Graph {
    fn get_node<N>(&self, index: usize) -> N;
    fn get_edge<E>(&self, index: usize) -> E;
}
