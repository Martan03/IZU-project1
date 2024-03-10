use std::fmt::Display;

/// Struct representing position
#[derive(Debug, Clone, PartialEq)]
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

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
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
    path: Vec<Pos>,
}

impl Node {
    /// Creates new [`Node`]
    pub fn new(pos: Pos, value: usize, path: Vec<Pos>) -> Self {
        Self { pos, value, path }
    }

    /// Gets [`Node`] position
    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    /// Gets [`Node`] value
    pub fn value(&self) -> usize {
        self.value
    }

    /// Gets [`Node`] path
    pub fn path(&self) -> &Vec<Pos> {
        &self.path
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}",
            self.path()
                .iter()
                .map(|pos| format!("{}", pos))
                .collect::<Vec<String>>()
                .join(", "),
            self.pos()
        )
    }
}
