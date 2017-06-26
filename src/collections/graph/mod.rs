/// Graph data structures and algorithms

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::cmp::Ord;
use std::collections::btree_map::{Entry, Keys};

pub use self::GraphErr::*;

pub enum GraphErr {
    NodeNotFound,
    LoopViolation,
    SelfLoopViolation,

}

pub type GraphResult<T> = Result<T, GraphErr>;

pub type NodeId = usize;

pub trait Graph<'a, N: 'a + Ord> {
    fn degree(&self, &N) -> GraphResult<usize>;

    fn in_degree(&self, &N) -> GraphResult<usize>;

    fn out_degree(&self, &N) -> GraphResult<usize>;

    fn is_directed(&self) -> bool;

    fn contains_node(&self, &N) -> bool;

    fn nodes(&'a self) -> Keys<'a, &'a N, Vec<&'a N>>;

    fn adjacent_nodes(&self, &N) -> GraphResult<&Vec<Rc<N>>>;

    fn predecessors(&self) -> NodeIterator<'a, N>;

    fn successors(&self) -> NodeIterator<'a, N>;

    fn edges(&'a self) -> EdgeIterator<'a, N>;

    fn is_connected(&self, &N, &N) -> bool;

    fn add_node(&mut self, n: &'a N) -> bool;

    fn add_edge(&mut self, head: &'a N, tail: &'a N) -> GraphResult<bool>;
}

pub struct NodeIterator<'a, N: 'a> {
    n: &'a N
}

#[derive(Clone)]
pub struct Edge<'a, N: 'a> {
    pub head: &'a N,
    pub tail: &'a N,
}

impl<'a, N: 'a> Iterator for NodeIterator<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<&'a N> {
        unimplemented!()
    }
}

pub struct EdgeIterator<'a, N: 'a> {
    edges: Vec<Edge<'a, N>>,
    index: usize,
}

impl<'a, N: 'a> Iterator for EdgeIterator<'a, N> {
    type Item = Edge<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.edges.len() {
            None
        } else {
            let edge = self.edges.get(self.index).unwrap();
            self.index +=1;
            Some(Edge{head: edge.head, tail: edge.tail})
        }
    }
}

pub struct GraphData<'a, N: 'a + Ord> {
    directed: bool,
    loop_allowed: bool,

    graph: BTreeMap<&'a N, Vec<&'a N>>
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

    pub fn build<N: 'static + Ord>(&self) -> GraphData<'static, N> {
        GraphData {
            directed: self.directed,
            loop_allowed: self.loop_allowed,
            graph: BTreeMap::new(),
        }
    }
}

impl<'a, N: 'a + Ord> Graph<'a, N> for GraphData<'a, N> {
    fn degree(&self, n: &N) -> GraphResult<usize> {

        if self.directed {
            let in_degree = self.in_degree(n);
            let out_degree = self.out_degree(n);

            if in_degree.is_ok() || out_degree.is_ok() {
                let mut degree = match in_degree {
                    Ok(d) => d,
                    Err(_) => 0
                };
                degree += match out_degree {
                    Ok(d) => d,
                    Err(_) => 0
                };

                return Ok(degree);
            } else {
                return in_degree;
            }
        } else {
            match self.graph.get(n) {
                Some(edges) => Ok(edges.len()),
                None => Err(NodeNotFound)
            }
        }
    }

    fn in_degree(&self, n: &N) -> GraphResult<usize> {
        unimplemented!()
    }

    fn out_degree(&self, n: &N) -> GraphResult<usize> {
        unimplemented!()
    }

    fn is_directed(&self) -> bool {
        self.directed
    }

    fn contains_node(&self, n: &N) -> bool {
        self.graph.contains_key(n)
    }

    fn nodes(&'a self) -> Keys<'a, &'a N, Vec<&'a N>> {
        self.graph.keys()
    }

    fn adjacent_nodes(&self, n: &N) -> GraphResult<&Vec<Rc<N>>> {
        unimplemented!()
    }

    fn predecessors(&self) -> NodeIterator<'a, N> {
        unimplemented!()
    }

    fn successors(&self) -> NodeIterator<'a, N> {
        unimplemented!()
    }

    fn edges(&'a self) -> EdgeIterator<'a, N> {
        let mut edges: Vec<Edge<'a, N>> = Vec::new();
        for tail in self.graph.keys() {
            match self.graph.get(tail) {
                Some(heads) => {
                    for head in heads {
                        edges.push(Edge{head: *head, tail: tail});
                    }
                }
                _ => panic!("Invalid graph")
            }
        }

        EdgeIterator { edges: edges, index: 0 }
    }

    fn is_connected(&self, n1: &N, n2: &N) -> bool {
        unimplemented!()
    }

    fn add_node(&mut self, n: &'a N) -> bool {
        match self.graph.entry(n) {
            Entry::Occupied(_) => false,
            Entry::Vacant(v) => {
                v.insert(Vec::new());
                true
            }
        }
    }

    fn add_edge(&mut self, head: &'a N, tail: &'a N) -> GraphResult<bool> {
        if self.directed {
            match self.graph.get_mut(tail) {
                Some(v) => {
                    if v.contains(&head) {
                        Ok(false)
                    } else {
                        v.push(head);
                        Ok(true)
                    }
                }
                None => Err(NodeNotFound)
            }
        } else {
            let mut added = false;
            match (self.graph.contains_key(tail), self.graph.contains_key(head)) {
                (true, true) => {
                    {
                        let v = self.graph.get_mut(tail).unwrap();
                        if !v.contains(&head) {
                            v.push(head);
                            added = true;
                        }
                    }
                    {
                        let v = self.graph.get_mut(head).unwrap();
                        if !v.contains(&tail) {
                            v.push(tail);
                            added = true;
                        }
                    }
                    Ok(added)
                }
                _ => return Err(NodeNotFound)
            }
        }
    }
}

#[test]
fn test_graph() {
    let mut builder = GraphBuilder::new();

    let a = "a".to_owned();
    let b = "b".to_owned();


    let mut g: GraphData<String> = builder.build();
    g.add_node(&a);
    g.add_node(&b);

    g.add_edge(&a, &b);

    for x in g.nodes() {
        println!("{}", x);
    }

    for e in g.edges() {
        println!("{} - {}", e.head, e.tail);
    }

    // {
    // g.add_node(&b);
    // }
    // {
    // g.add_edge(&a, &b);
    // }

}