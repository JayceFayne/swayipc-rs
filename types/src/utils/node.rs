use crate::reply::Node;

impl Node {
    pub fn find_as_ref(&self, predicate: fn(&Node) -> bool) -> Option<&Node> {
        if predicate(self) {
            return Some(self);
        }
        for node in &self.nodes {
            let node = node.find_as_ref(predicate);
            if node.is_some() {
                return node;
            }
        }
        for node in &self.floating_nodes {
            let node = node.find_as_ref(predicate);
            if node.is_some() {
                return node;
            }
        }
        None
    }

    pub fn find(self, predicate: fn(&Node) -> bool) -> Option<Node> {
        if predicate(&self) {
            return Some(self);
        }
        let Node {
            nodes,
            floating_nodes,
            ..
        } = self;
        for node in nodes {
            let node = node.find(predicate);
            if node.is_some() {
                return node;
            }
        }
        for node in floating_nodes {
            let node = node.find(predicate);
            if node.is_some() {
                return node;
            }
        }
        None
    }

    pub fn find_focused_as_ref(&self, predicate: fn(&Node) -> bool) -> Option<&Node> {
        if predicate(self) {
            return Some(self);
        }
        if self.focus.is_empty() {
            return None;
        }
        let first = self.focus[0];
        for node in &self.nodes {
            if node.id == first {
                return node.find_focused_as_ref(predicate);
            }
        }
        for node in &self.floating_nodes {
            if node.id == first {
                return node.find_focused_as_ref(predicate);
            }
        }
        None
    }

    pub fn find_focused(self, predicate: fn(&Node) -> bool) -> Option<Node> {
        if predicate(&self) {
            return Some(self);
        }
        let Node {
            nodes,
            floating_nodes,
            focus,
            ..
        } = self;
        if focus.is_empty() {
            return None;
        }
        let first = focus[0];
        for node in nodes {
            if node.id == first {
                return node.find_focused(predicate);
            }
        }
        for node in floating_nodes {
            if node.id == first {
                return node.find_focused(predicate);
            }
        }
        None
    }
}
