use std::{fs::File, io::Read};

use crate::node::{NodeType, Pos};

/// Struct representing transition map
#[derive(Debug, Clone)]
pub struct TransMap {
    board: Vec<Vec<NodeType>>,
}

impl TransMap {
    /// Loads map from the file and parses it
    /// Excepts valid map
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Self {
            board: TransMap::parse_string(contents),
        })
    }

    /// Gets item on given position from board
    pub fn get(&self, pos: &Pos) -> &NodeType {
        self.board
            .get(pos.y)
            .and_then(|row| row.get(pos.x))
            .unwrap_or(&NodeType::Wall)
    }

    /// Parses string to the board
    /// Expects valid map
    fn parse_string(contents: String) -> Vec<Vec<NodeType>> {
        let lines: Vec<&str> = contents.lines().collect();
        let non_id_lines = &lines[1..];

        non_id_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|item| {
                        if let Ok(num) = item.parse::<usize>() {
                            NodeType::Node(num)
                        } else {
                            NodeType::Wall
                        }
                    })
                    .collect()
            })
            .collect()
    }
}
