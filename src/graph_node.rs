use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct GraphEdge<E> {
    pub data: E,
    pub node: usize,
}

impl<E> GraphEdge<E> {
    pub fn new(data: E, node: usize) -> Self {
        Self { data, node }
    }
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct GraphNode<D, E> {
    pub data: D,
    pub edges: Vec<GraphEdge<E>>,
}

impl<D, E> GraphNode<D, E> {
    pub fn new(data: D) -> Self {
        Self {
            data,
            edges: Vec::new(),
        }
    }
}
