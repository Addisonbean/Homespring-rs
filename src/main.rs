extern crate homespring;

use homespring::RiverNode;
use homespring::Tick;

fn main() {

    // let river = RiverNode::parse_program("a b c");
    let river = RiverNode::parse_program("Universe bear snowmelt");

    let mut river = river.borrow_mut();

    river.tick(Tick::Snow);

    // println!("{:?}", river.node_type);
    // let child = river.borrow_child(0);
    // println!("{:?}", child.node_type);
    // println!("{:?}", child.borrow_child(0).node_type);

}
