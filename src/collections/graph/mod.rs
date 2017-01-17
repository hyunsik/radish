/// Graph data structures and algorithms

use std::collections::{HashMap, HashSet};

pub type NodeId = usize;

pub trait Graph<'a, V: 'a> {
    fn degree(&self, &V) -> usize;

    fn in_degree(&self, &V) -> usize;

    fn out_degree(&self, &V) -> usize;

    fn is_directed(&self) -> bool;

    fn contains_node(&self, &V) -> bool;

    fn nodes(&self) -> NodeIterator<'a, V>;

    fn adjacent_nodes(&self, &V) -> NodeIterator<'a, V>;

    fn predecessors(&self) -> NodeIterator<'a, V>;

    fn successors(&self) -> NodeIterator<'a, V>;

    fn edges(&self);

    fn is_connected(&self, v1: &V, v2: &V) -> bool;
}

pub trait MutableGraph<'a, V: 'a>: Graph<'a, V> {
    fn add_edge(&mut self, &V, &V);
}

pub trait ValueGraph<'a, V: 'a, E: 'a> : Graph<'a, V> {
    fn edge(&self, v1: &V, v2: &V) -> &E;
}

pub trait MutableValueGraph<'a, V: 'a, E: 'a>: ValueGraph<'a, V, E> {
    fn add_edge(&mut self, &V, &V, e: E);
}

pub struct NodeIterator<'a, V: 'a> {
    v: &'a V
}

impl<'a, V: 'a> Iterator for NodeIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        unimplemented!()
    }
}

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

impl GraphBuilder {
    pub fn new() -> GraphBuilder {
        GraphBuilder {
            directed: false,
            loop_allowed: false,
        }
    }

    pub fn set_directed(mut self, directed: bool) -> GraphBuilder {
        self.directed = directed;
        self
    }

    pub fn allow_loop(mut self, loop_allowed: bool) -> GraphBuilder {
        self.loop_allowed = loop_allowed;
        self
    }

    pub fn build<V: 'static>(&self) -> Box<Graph<V>> {
        Box::new(GraphData {
            directed: self.directed,
            loop_allowed: self.loop_allowed,
            nodes: HashMap::new(),
            edges: HashMap::new()
        })
    }

    pub fn build_with_labeled_edge<V, E>(&self) -> Box<ValueGraph<V, E>> {
        unimplemented!()
    }
}

impl<'a, V: 'a> Graph<'a, V> for GraphData<V> {
    fn degree(&self, v: &V) -> usize {
        unimplemented!()
    }

    fn in_degree(&self, v: &V) -> usize {
        unimplemented!()
    }

    fn out_degree(&self, v: &V) -> usize {
        unimplemented!()
    }

    fn is_directed(&self) -> bool {
        self.directed
    }

    fn contains_node(&self, v: &V) -> bool {
        unimplemented!()
    }

    fn nodes(&self) -> NodeIterator<'a, V> {
        unimplemented!()
    }

    fn adjacent_nodes(&self, v: &V) -> NodeIterator<'a, V> {
        unimplemented!()
    }

    fn predecessors(&self) -> NodeIterator<'a, V> {
        unimplemented!()
    }

    fn successors(&self) -> NodeIterator<'a, V> {
        unimplemented!()
    }

    fn edges(&self) {
        unimplemented!()
    }

    fn is_connected(&self, v1: &V, v2: &V) -> bool {
        unimplemented!()
    }
}