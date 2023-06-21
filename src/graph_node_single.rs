use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct GraphEdgeSingle<D> {
    pub data: D,
    pub node: usize,
}

impl<D> GraphEdgeSingle<D> {
    pub fn new(data: D, node: usize) -> Self {
        Self { data, node }
    }
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct GraphNodeSingle<D> {
    pub data: D,
    pub edges: Vec<GraphEdgeSingle<D>>,
}

impl<D> GraphNodeSingle<D> {
    pub fn new(data: D) -> Self {
        Self {
            data,
            edges: Vec::new(),
        }
    }
}
