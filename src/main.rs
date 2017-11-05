extern crate homespring;

use homespring::RiverNode;

fn main() {

    let river = RiverNode::parse_program("a b c");

    let river = river.borrow();
    println!("{:?}", river);
    println!("{:?}", river.children[0].borrow().node_type);
    let child = river.children[0].borrow();
    println!("{:?}", child.children[0].borrow().node_type);

}
