use std::collections::HashSet;
use std::iter::FromIterator;

use mask::{CpuMask, NodeMask};
use node::Node;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodeset_from_iter() {
        let mut set: NodeSet = [0].iter().map(|i| Node::new(*i)).collect();
        assert_eq!(2 + 2, 4);
    }

}

#[derive(Debug)]
pub struct CpuSet(HashSet<u64>);
#[derive(Debug)]
pub struct NodeSet(HashSet<Node>);

impl CpuSet {
    pub fn new() -> CpuSet {
        CpuSet(HashSet::new())
    }
}

impl IntoIterator for CpuSet {
    type Item = <HashSet<u64> as IntoIterator>::Item;
    type IntoIter = <HashSet<u64> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<CpuMask> for CpuSet {
    fn from(m: CpuMask) -> CpuSet {
        let mut s = CpuSet::new();
        for i in 0..m.len() {
            if m.is_set(i) {
                s.0.insert(i as u64);
            }
        }
        s
    }
}

impl NodeSet {
    pub fn new() -> NodeSet {
        NodeSet(HashSet::new())
    }

    pub fn contains(&self, value: &Node) -> bool {
        self.0.contains(value)
    }
}

impl IntoIterator for NodeSet {
    type Item = <HashSet<Node> as IntoIterator>::Item;
    type IntoIter = <HashSet<Node> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Node> for NodeSet {
    fn from_iter<I: IntoIterator<Item=Node>>(iter: I) -> Self {
        let mut s = NodeSet::new();
        for i in iter {
            s.0.insert(i);
        }
        s
    }
} 

impl From<NodeMask> for NodeSet {
    fn from(m: NodeMask) -> NodeSet {
        let mut s = NodeSet::new();
        for i in 0..m.len() {
            if m.is_set(i) {
                let node = Node::new(i as i32);
                s.0.insert(node);
            }
        }
        s
    }
}