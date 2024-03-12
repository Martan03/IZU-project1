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

    /// Prints the whole path of the [`Node`]
    pub fn print_res(&self) {
        println!(
            "{}, {}",
            self.path
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.pos
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = self.path.last() {
            write!(f, "({}, {}, {})", self.pos, self.value, parent)
        } else {
            write!(f, "({}, {}, null)", self.pos, self.value)
        }
    }
}
