/// Struct representing position
#[derive(Debug, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    /// Creates new [`Pos`] with given coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Represents node type
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Node(usize),
    Wall,
}

/// Struct representing Node, containing path needed to get there
#[derive(Debug, Clone)]
pub struct Node {
    pos: Pos,
    value: usize,
    path: Vec<Node>,
}

impl Node {
    /// Creates new [`Node`]
    pub fn new(pos: Pos, value: usize, path: Vec<Node>) -> Self {
        Self { pos, value, path }
    }
}
