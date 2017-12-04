extern crate homespring;

use homespring::river::{Node, NodeType};

#[test]
fn basic_program() {
    let river = Node::parse_program("a b c").to_node().unwrap();
    let root_node = river.borrow();

    assert_eq!(root_node.node_type, NodeType::Other("a".to_owned()));
    assert_eq!(root_node.children.len(), 1);

    let child = root_node.borrow_child(0);
    assert_eq!(child.node_type, NodeType::Other("b".to_owned()));

    let sub_child = child.borrow_child(0);
    assert_eq!(sub_child.node_type, NodeType::Other("c".to_owned()));
    assert_eq!(sub_child.children.len(), 0);
}

#[test]
fn empty_nodes() {
    let river = Node::parse_program("a b  c  d e").to_node().unwrap();
    let root_node = river.borrow();

    let c = root_node.borrow_child(1);
    let d = root_node.borrow_child(2);
    let e = d.borrow_child(0);

    assert_eq!(c.node_type, NodeType::Other("c".to_owned()));
    assert_eq!(d.node_type, NodeType::Other("d".to_owned()));
    assert_eq!(e.node_type, NodeType::Other("e".to_owned()));
}

