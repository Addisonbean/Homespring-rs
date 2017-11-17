extern crate homespring;

use homespring::river::{Node, NodeType};

#[test]
fn basic_program() {
    let river = Node::parse_program("a b c");
    let root_node = river.borrow();

    assert_eq!(root_node.node_type, NodeType::Other("a".to_owned()));
    assert_eq!(root_node.children.len(), 1);

    let child = root_node.borrow_child(0);
    assert_eq!(child.node_type, NodeType::Other("b".to_owned()));

    let sub_child = child.borrow_child(0);
    assert_eq!(sub_child.node_type, NodeType::Other("c".to_owned()));
    assert_eq!(sub_child.children.len(), 0);
}

