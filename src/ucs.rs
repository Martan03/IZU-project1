use crate::{
    node::{Node, NodeType, Pos},
    trans_map::TransMap,
};

/// Struct implementing Unified cost search
pub struct UCS {
    trans_map: TransMap,
    end: Pos,
    open: Vec<Node>,
    closed: Vec<Node>,
}

impl UCS {
    /// Creates new [`UCS`]
    pub fn new(
        trans_map: TransMap,
        start: Pos,
        end: Pos,
    ) -> Result<Self, String> {
        if trans_map.get(&start) == &NodeType::Wall {
            return Err("Start cannot be Wall".to_owned());
        }
        if trans_map.get(&end) == &NodeType::Wall {
            return Err("End cannot be Wall".to_owned());
        }

        let open = vec![Node::new(start, 0, vec![])];
        Ok(Self {
            trans_map,
            end,
            open,
            closed: vec![],
        })
    }

    /// Finds the path with lowest value
    pub fn search(&mut self) -> Result<Node, String> {
        let mut iter = 0;
        while !self.open.is_empty() {
            self.print(iter);
            let item = self.open.remove(0);
            if item.pos() == &self.end {
                return Ok(item);
            }

            self.expand(&item);
            iter += 1;
        }

        Err("End cannot be reached".to_owned())
    }

    fn expand(&mut self, node: &Node) {
        self.closed.push(node.clone());
        let pos = node.pos();

        self._expand(pos.x.saturating_sub(1), pos.y.saturating_sub(1), node);
        self._expand(pos.x, pos.y.saturating_sub(1), node);
        self._expand(pos.x + 1, pos.y.saturating_sub(1), node);

        self._expand(pos.x.saturating_sub(1), pos.y, node);
        self._expand(pos.x + 1, pos.y, node);

        self._expand(pos.x.saturating_sub(1), pos.y + 1, node);
        self._expand(pos.x, pos.y + 1, node);
        self._expand(pos.x + 1, pos.y + 1, node);
    }

    /// Helper function for node expanding
    fn _expand(&mut self, x: usize, y: usize, node: &Node) {
        let pos = Pos::new(x, y);
        let node_type = self.trans_map.get(&pos);
        self.add(node_type.clone(), pos, node);
    }

    /// Adds node to the open queue
    fn add(&mut self, node_type: NodeType, pos: Pos, cur: &Node) {
        if self.closed.iter().any(|n| n.pos() == &pos) {
            return;
        }

        let NodeType::Node(value) = node_type else {
            return;
        };

        let mut path = cur.path().clone();
        path.push(cur.pos().clone());
        let exist_id = self.open.iter().position(|n| n.pos() == &pos);
        let node = Node::new(pos, value + cur.value(), path);

        match exist_id {
            Some(id) => {
                if self.open[id].value() > node.value() {
                    self.open.remove(id);
                    self._add(node);
                }
            }
            None => self._add(node),
        }
    }

    /// Helper function for adding item
    fn _add(&mut self, node: Node) {
        let index = self
            .open
            .iter()
            .position(|n| node.value() < n.value())
            .unwrap_or(self.open.len());
        self.open.insert(index, node);
    }

    fn print(&self, iter: usize) {
        println!("{iter}. iterace");
        println!(
            "Open:\n{}",
            self.open
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Closed:\n{}\n",
            self.closed
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}
