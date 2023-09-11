use crate::reply::Node;

impl Node {
    pub fn find_as_ref<F>(&self, predicate: F) -> Option<&Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
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

    pub fn find<F>(self, predicate: F) -> Option<Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
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

    pub fn find_focused_as_ref<F>(&self, predicate: F) -> Option<&Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
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

    pub fn find_focused<F>(self, predicate: F) -> Option<Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
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

    pub fn iter(&self) -> NodeIterator<'_> {
        NodeIterator { queue: vec![self] }
    }
}

pub struct NodeIterator<'a> {
    queue: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        match self.queue.pop() {
            None => None,
            Some(result) => {
                self.queue.extend(result.nodes.iter());
                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeBorder;
    use crate::NodeLayout;
    use crate::NodeType;
    use crate::Rect;

    fn rect() -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }
    }

    fn mk_node(name: Option<String>, node_type: NodeType, nodes: Vec<Node>) -> Node {
        Node {
            id: 1,
            name,
            node_type,
            border: NodeBorder::Normal,
            current_border_width: 0,
            layout: NodeLayout::Tabbed,
            percent: None,
            rect: rect(),
            window_rect: rect(),
            deco_rect: rect(),
            geometry: rect(),
            urgent: false,
            focused: false,
            focus: Vec::new(),
            nodes,
            floating_nodes: Vec::new(),
            sticky: false,
            representation: None,
            fullscreen_mode: None,
            app_id: None,
            pid: None,
            window: None,
            num: None,
            window_properties: None,
            marks: Vec::new(),
            inhibit_idle: None,
            idle_inhibitors: None,
            shell: None,
            visible: None,
            output: None,
        }
    }

    #[test]
    fn returns_the_given_root_node_first() {
        let root = mk_node(Some(String::from("root")), NodeType::Root, vec![]);
        let mut iterator = root.iter();
        assert_eq!(iterator.next().unwrap().name, Some("root".to_string()));
    }

    #[test]
    fn returns_children_of_the_given_node() {
        let root = mk_node(
            Some("root".to_string()),
            NodeType::Root,
            vec![mk_node(Some("child".to_string()), NodeType::Con, vec![])],
        );
        let mut iterator = root.iter();
        iterator.next();
        assert_eq!(iterator.next().unwrap().name, Some("child".to_string()));
    }

    #[test]
    fn returns_transitive_children_of_the_given_node() {
        let root = mk_node(
            Some("root".to_string()),
            NodeType::Root,
            vec![mk_node(
                Some("child".to_string()),
                NodeType::Con,
                vec![mk_node(
                    Some("grandchild".to_string()),
                    NodeType::Con,
                    vec![],
                )],
            )],
        );
        let mut iterator = root.iter();
        iterator.next();
        iterator.next();
        assert_eq!(
            iterator.next().unwrap().name,
            Some("grandchild".to_string())
        );
    }

    #[test]
    fn returns_none_at_the_end() {
        let root = mk_node(Some("root".to_string()), NodeType::Root, vec![]);
        let mut iterator = root.iter();
        iterator.next();
        assert_eq!(iterator.next(), None);
    }
}
