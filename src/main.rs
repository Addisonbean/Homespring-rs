extern crate homespring;

use homespring::RiverNode;
use homespring::Tick;

fn main() {

    let river = RiverNode::parse_program("Universe bear  snowmelt");
    let mut river = river.borrow_mut();

    println!("{:?}", river.snow);
    println!("{:?}", river.borrow_child(1).snow);

    river.tick(Tick::Snow);

    println!("{:?}", river.snow);
    println!("{:?}", river.borrow_child(1).snow);

}
