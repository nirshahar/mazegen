pub trait UnionFind<S> {
    fn add_element(&mut self) -> S;
    fn union(&mut self, first_node: S, second_node: S);
    fn find(&mut self, node: S) -> S;
}

pub trait UnionFindSetCmp<S> {
    fn is_in_same_set(&mut self, first_node: S, second_node: S) -> bool;
}

impl<T, S> UnionFindSetCmp<S> for T
where
    T: UnionFind<S>,
    S: Eq,
{
    fn is_in_same_set(&mut self, first_node: S, second_node: S) -> bool {
        let first_set = self.find(first_node);
        let second_set = self.find(second_node);

        first_set == second_set
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIdx(usize);

impl std::ops::Deref for NodeIdx {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    idx: NodeIdx,
    parent_idx: NodeIdx,
    size: u32,
}

impl Node {
    fn new(idx: NodeIdx) -> Self {
        Self {
            idx: idx,
            parent_idx: idx,
            size: 1,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl Eq for Node {}

#[derive(Debug)]
pub struct A {
    nodes: Vec<Node>,
}

impl A {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl UnionFind<NodeIdx> for A {
    fn add_element(&mut self) -> NodeIdx {
        let idx = NodeIdx(self.nodes.len());
        self.nodes.push(Node::new(idx));

        idx
    }

    fn union(&mut self, first_node: NodeIdx, second_node: NodeIdx) {
        let first_set_idx = self.find(first_node);
        let second_set_idx = self.find(second_node);

        let mut first_set = self.nodes[first_set_idx.0];
        let mut second_set = self.nodes[second_set_idx.0];

        if first_set.idx == second_set.idx {
            return;
        }

        if second_set.size < first_set.size {
            (first_set, second_set) = (second_set, first_set);
        }

        self.nodes[first_set.idx.0].parent_idx = second_set.idx;
        self.nodes[second_set.idx.0].size += first_set.size;
    }

    fn find(&mut self, node: NodeIdx) -> NodeIdx {
        let mut node = self.nodes[node.0];

        let mut nodes_to_update = Vec::new();

        while node.idx != node.parent_idx {
            nodes_to_update.push(node);
            node = self.nodes[node.parent_idx.0];
        }

        let parent = node;

        for node in nodes_to_update {
            self.nodes[node.parent_idx.0].size -= node.size;
            self.nodes[node.idx.0].parent_idx = parent.idx;
        }

        parent.idx
    }
}
