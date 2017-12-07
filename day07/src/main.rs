extern crate day07;
use day07::Node;

extern crate util;
use util::read_file;

fn main() {
    let input_contents = read_file("input.txt");
    let nodes = Node::from_input(&input_contents);
    // ensure there is exactly one root node
    assert_eq!(nodes.iter().filter(|node| !node.parent_exists()).count(), 1);
    let root = nodes
        .iter()
        .filter(|node| !node.parent_exists())
        .next()
        .unwrap();
    println!("Name of root node: {}", root.name);
    for (unbalanced_node, children) in root.unbalanced_siblings() {
        println!(
            "Found unbalanced node {} with child weights:",
            unbalanced_node
        );
        for (name, weight, total_weight) in children {
            println!("  {}: {} ({})", name, weight, total_weight);
        }
    }
}
