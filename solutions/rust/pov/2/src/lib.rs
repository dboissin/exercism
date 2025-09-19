use std::fmt::Debug;

#[derive(Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    nodes: Vec<Node<T>>,
    root: usize,
}

impl<T: Debug + Ord> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.nodes[self.root].value == other.nodes[other.root].value &&
        self.nodes.len() == other.nodes.len() &&
        self.nodes.iter().all(|item| {
            if let Some(other_item) = other.nodes.iter().find(|o_item| *o_item == item) {
                let self_children:Vec<&T> = item.children.iter().map(|idx| &self.nodes[*idx].value).collect();
                let other_children:Vec<&T> = other_item.children.iter().map(|idx| &other.nodes[*idx].value).collect();
                self_children.iter().all(|el| other_children.contains(el))
            } else {
                false
            }
        })
    }
}

impl<T: Debug + Ord> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_traverse(f, self.root, &mut Vec::new());
        Ok(())
    }
}

#[derive(Debug, Eq, PartialOrd, Ord)]
struct Node<T: Debug + Ord> {
    value: T,
    children: Vec<usize>,
}

impl<T: Debug + Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.children.len() == other.children.len()
    }
}

impl<T: Debug + Ord> Node<T> {

    pub fn new(label: T) -> Self {
        Node { value: label, children: vec![] }
    }

    fn incr_idx(&mut self, incr_value: usize) {
        self.children = self.children.iter().map(|x| x + incr_value).collect();
    }
}


impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Tree { nodes: vec![Node::new(label)], root: 0 }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        let orig_len = self.nodes.len();
        for mut node in child.nodes {
            node.incr_idx(orig_len);
            self.nodes.push(node);
        }
        self.nodes[self.root].children.push(orig_len + child.root);
        self
    }

    fn write_traverse(&self, f: &mut std::fmt::Formatter<'_>, idx: usize, visited: &mut Vec<usize>) {
        self.write_node(f, idx);
        for node in &self.nodes[idx].children {
            if !visited.contains(node) {
                visited.push(*node);
                self.write_traverse(f, *node, visited);
            }
        }
    }

    fn write_node(&self, f: &mut std::fmt::Formatter<'_>, idx: usize) {
        let _ = write!(f, "\n{:?} -->", self.nodes[idx].value);
        for n in &self.nodes[idx].children {
            let _ = write!(f, " {:?}", self.nodes[*n].value);
        }
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        if self.nodes[self.root].value == *from {
            return true;
        }
        if let Some(new_root_idx) = self.nodes.iter().position(|node| node.value == *from) {
            self.root = new_root_idx;
            let mut visited = vec![new_root_idx];
            let mut curr_idx = new_root_idx;
            while let Some(node_idx) = self.nodes.iter().enumerate().position(|(idx, node)| node.children.contains(&curr_idx) && !visited.contains(&idx)) {
                let remove_idx = self.nodes[node_idx].children.iter().position(|idx| *idx == curr_idx).unwrap();
                self.nodes[node_idx].children.remove(remove_idx);
                self.nodes[curr_idx].children.push(node_idx);
                curr_idx = node_idx;
                visited.push(curr_idx);
            }
            return true;
        }
        false
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if self.pov_from(to) {
            let mut path = vec![];
            if self.path(from, &mut path, self.root) {
                path.push(self.root);
                return Some(path.iter().map(|idx| &self.nodes[*idx].value).collect::<Vec<&'a T>>());
            }
        }
        None
    }

    fn path(&self, to: &T, path: &mut Vec<usize>, idx: usize) -> bool {
        if self.nodes[idx].value == *to {
            true
        } else {
            for child_idx in &self.nodes[idx].children {
                if !path.contains(child_idx) && self.path(to, path, *child_idx) {
                    path.push(*child_idx);
                    return true;
                }
            }
            false
        }
    }

}


#[cfg(test)]
mod node_tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn find_node_eq() {
        let a = Node::new("a");
        let b = Node::new("a");
        assert_eq!(a, b);
    }

    #[test]
    fn pov_node_test() {
        let tree1 = Tree::new("a")
            .with_child(Tree::new("b"))
            .with_child(Tree::new("c").with_child(Tree::new("d")));
        let tree2 = Tree::new("a")
            .with_child(Tree::new("c").with_child(Tree::new("d")))
            .with_child(Tree::new("b"));
        assert_eq!(tree1, tree2);
    }

}
