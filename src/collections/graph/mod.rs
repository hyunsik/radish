use std::collections::{HashMap, HashSet};

pub struct NodeIterator<'a, V: 'a> {
    v: &'a V
}

impl<'a, V: 'a> Iterator for NodeIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        unimplemented!()
    }
}

pub type NodeId = usize;

pub struct GraphData<V> {
    directed: bool,
    loop_allowed: bool,

    nodes: HashMap<NodeId, V>,
    edges: HashMap<NodeId, NodeId>
}

pub struct ValueGraphData<V, E> {
    nodes: HashMap<NodeId, V>,
    edges: HashMap<NodeId, NodeId>,
    edge_labels: HashMap<(NodeId, NodeId),E>
}

pub struct GraphBuilder {
    directed: bool,
    loop_allowed: bool
}

pub trait Graph<V> {
    fn degree(&self, &V) -> usize;

    fn in_degree(&self, &V) -> usize;

    fn out_degree(&self, &V) -> usize;

    fn is_directed(&self) -> bool;

    fn contains_node(&self, &V) -> bool;

    fn nodes<'a>(&'a self) -> NodeIterator<'a, V>;

    fn adjacent_nodes<'a>(&'a self, &V) -> NodeIterator<'a, V>;

    fn predecessors<'a>(&'a self) -> NodeIterator<'a, V>;

    fn successors<'a>(&'a self) -> NodeIterator<'a, V>;

    fn edges<'a>(&'a self);

    fn is_connected(&self, v1: &V, v2: &V) -> bool;
}

pub trait MutableGraph<V>: Graph<V> {
    fn add_edge(&mut self, &V, &V);
}

pub trait ValueGraph<V, E> : Graph<V> {
    fn edge(&self, v1: &V, v2: &V) -> &E;
}

pub trait MutableValueGraph<V, E>: ValueGraph<V, E> {
    fn add_edge(&mut self, &V, &V, e: E);
}