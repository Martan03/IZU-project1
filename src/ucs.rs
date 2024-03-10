use crate::{
    node::{Node, NodeType, Pos},
    trans_map::TransMap,
};

/// Struct implementing Unified cost search
pub struct UCS {
    trans_map: TransMap,
    end: Pos,
    open: Vec<Node>,
    closed: Vec<Pos>,
}

impl UCS {
    /// Creates new [`UCS`]
    pub fn new(
        trans_map: TransMap,
        start: Pos,
        end: Pos,
    ) -> Result<Self, String> {
        let NodeType::Node(start_val) = trans_map.get(&start) else {
            return Err("Start cannot be Wall".to_owned());
        };
        if trans_map.get(&end) == &NodeType::Wall {
            return Err("End cannot be Wall".to_owned());
        }

        let open = vec![Node::new(start.clone(), *start_val, vec![])];
        let closed = vec![start];
        Ok(Self {
            trans_map,
            end,
            open,
            closed,
        })
    }

    /// Finds the path with lowest value
    pub fn search(&mut self) -> Result<Node, String> {
        let mut iter = 1_usize;
        while !self.open.is_empty() {
            let item = self.open.remove(0);
            println!("Iteration: {}", iter);
            println!("{}", item);
            if item.pos() == &self.end {
                return Ok(item);
            }

            self.expand(&item);
            iter += 1;
        }

        Err("End cannot be reached".to_owned())
    }

    fn expand(&mut self, node: &Node) {
        let pos = node.pos();

        let expanded_pos = Pos::new(pos.x, pos.y.saturating_sub(1));
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos =
            Pos::new(pos.x.saturating_sub(1), pos.y.saturating_sub(1));
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x + 1, pos.y.saturating_sub(1));
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x.saturating_sub(1), pos.y);
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x + 1, pos.y);
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x.saturating_sub(1), pos.y + 1);
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x, pos.y + 1);
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);

        let expanded_pos = Pos::new(pos.x + 1, pos.y + 1);
        let node_type = self.trans_map.get(&expanded_pos);
        self.add(node_type.clone(), expanded_pos.clone(), node);
        self.closed.push(expanded_pos);
    }

    fn add(&mut self, node_type: NodeType, pos: Pos, cur: &Node) {
        if self.closed.contains(&pos) {
            return;
        }

        let NodeType::Node(value) = node_type else {
            return;
        };

        let mut path = cur.path().clone();
        path.push(cur.pos().clone());
        let node = Node::new(pos, value + cur.value(), path);

        let index = self
            .open
            .iter()
            .position(|n| node.value() < n.value())
            .unwrap_or(self.open.len());
        self.open.insert(index, node);
    }
}
