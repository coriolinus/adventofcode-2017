use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub mod parser; // synthesized by lalrpop

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub name: &'a str,
    weight: usize,
    children: RefCell<Vec<Rc<Node<'a>>>>,
    parent: RefCell<Option<Weak<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, weight: usize, carried_len: usize) -> Node<'a> {
        Node {
            name: name,
            weight: weight,
            children: RefCell::new(Vec::with_capacity(carried_len)),
            parent: RefCell::new(None),
        }
    }

    pub fn from_input(input_contents: &'a str) -> Vec<Rc<Node<'a>>> {
        let mut nodes = HashMap::new();
        let mut carried_map = HashMap::new();

        // start by creating nodes for every line in the input
        for line in input_contents.lines() {
            if let Ok((name, weight, carried)) = parser::parse_Node(line) {
                nodes.insert(name, Rc::new(Node::new(name, weight, carried.len())));
                carried_map.insert(name, carried);
            } else {
                panic!("Failed to parse {}", line);
            }
        }

        // now backfill the references to carried nodes
        for (name, carried_names) in carried_map.iter() {
            // first, add the children
            {
                // unwrap is safe because every key in carried_map must also
                // be present in nodes
                let mut node_children = nodes.get(name).unwrap().children.borrow_mut();
                for child_name in carried_names.iter() {
                    let child = nodes.get(child_name).expect(
                        "Carried node must be in nodes list",
                    );
                    node_children.push(child.clone());
                }
            }
            // now, add the parents
            {
                // unwrap is safe because every key in carried_map must also
                // be present in nodes
                let node = nodes.get(name).unwrap();
                for child_name in carried_names.iter() {
                    let mut child_parent = nodes
                        .get(child_name)
                        .expect("Carried node must be in nodes list")
                        .parent
                        .borrow_mut();
                    *child_parent = Some(Rc::downgrade(&node.clone()));
                }
            }
        }

        let mut rv = Vec::with_capacity(nodes.len());
        rv.extend(nodes.drain().map(|(_, node_ref)| node_ref));
        rv
    }

    pub fn parent_exists(&self) -> bool {
        self.parent.borrow().is_some()
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn children_weight(&self) -> usize {
        self.children
            .borrow()
            .iter()
            .map(|child| child.get_total_weight())
            .sum()
    }

    pub fn get_total_weight(&self) -> usize {
        self.weight + self.children_weight()
    }

    pub fn children_are_unbalanced(&self) -> bool {
        !self.children.borrow().is_empty() &&
            {
                let first_child_weight = self.children.borrow().first().unwrap().get_total_weight();
                self.children.borrow().iter().any(|child| {
                    child.get_total_weight() != first_child_weight
                })
            }
    }

    fn unbalanced_siblings_interior(&self, rv: &mut Vec<(&'a str, Vec<(&'a str, usize, usize)>)>) {
        if self.children_are_unbalanced() {
            rv.push((
                self.name,
                self.children
                    .borrow()
                    .iter()
                    .map(|child| (child.name, child.weight, child.get_total_weight()))
                    .collect(),
            ));
        }
        for child in self.children.borrow().iter() {
            child.unbalanced_siblings_interior(rv);
        }
    }

    pub fn unbalanced_siblings(&self) -> Vec<(&'a str, Vec<(&'a str, usize, usize)>)> {
        let mut rv = Vec::new();
        self.unbalanced_siblings_interior(&mut rv);
        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "
        pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)
        "
            .trim()
    }

    fn big_example() -> String {
        let mut ex = example().to_string();
        ex += "
        bleh (778)
        blah (271) -> tknk, bleh";
        ex
    }

    #[test]
    fn test_tknk_total_weight() {
        let nodes = Node::from_input(example());
        // ensure there is exactly one root node
        assert_eq!(nodes.iter().filter(|node| !node.parent_exists()).count(), 1);
        let root = nodes
            .iter()
            .filter(|node| !node.parent_exists())
            .next()
            .unwrap();
        // ensure the correct node is detected
        assert_eq!(root.name, "tknk");
        assert_eq!(root.get_weight(), 41);
        assert_eq!(root.children_weight(), 737);
        assert_eq!(root.get_total_weight(), 778);
    }

    #[test]
    fn test_example_part_1() {
        let nodes = Node::from_input(example());
        // ensure there is exactly one root node
        assert_eq!(nodes.iter().filter(|node| !node.parent_exists()).count(), 1);
        let root = nodes
            .iter()
            .filter(|node| !node.parent_exists())
            .next()
            .unwrap();
        // ensure the correct node is detected
        assert_eq!(root.name, "tknk");
    }

    #[test]
    fn test_example_part_2() {
        let nodes = Node::from_input(example());
        // ensure there is exactly one root node
        assert_eq!(nodes.iter().filter(|node| !node.parent_exists()).count(), 1);
        let root = nodes
            .iter()
            .filter(|node| !node.parent_exists())
            .next()
            .unwrap();

        let unbalanced_siblings = root.unbalanced_siblings();
        for &(ref unbalanced_node, ref children) in unbalanced_siblings.iter() {
            println!(
                "Found unbalanced node {} with child weights:",
                unbalanced_node
            );
            for &(ref name, ref weight, ref total_weight) in children {
                println!("  {}: {} ({})", name, weight, total_weight);
            }
        }

        assert_eq!(unbalanced_siblings.len(), 1);
        assert_eq!(unbalanced_siblings[0].0, "tknk");
    }

    #[test]
    fn test_example_part_2_big() {
        let biggy = big_example();
        let nodes = Node::from_input(&biggy);
        // ensure there is exactly one root node
        assert_eq!(nodes.iter().filter(|node| !node.parent_exists()).count(), 1);
        let root = nodes
            .iter()
            .filter(|node| !node.parent_exists())
            .next()
            .unwrap();

        assert_eq!(root.name, "blah");

        let unbalanced_siblings = root.unbalanced_siblings();
        for &(ref unbalanced_node, ref children) in unbalanced_siblings.iter() {
            println!(
                "Found unbalanced node {} with child weights:",
                unbalanced_node
            );
            for &(ref name, ref weight, ref total_weight) in children {
                println!("  {}: {} ({})", name, weight, total_weight);
            }
        }

        assert_eq!(unbalanced_siblings.len(), 1);
        assert_eq!(unbalanced_siblings[0].0, "tknk");
    }
}
