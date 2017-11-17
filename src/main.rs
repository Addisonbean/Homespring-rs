extern crate homespring;

use homespring::river::Node;
use homespring::Tick;

fn main() {

    let river = Node::parse_program("Universe hatchery  snowmelt");;
    let mut river = river.borrow_mut();

    println!("{:?}", river.snow);
    println!("{:?}", river.borrow_child(1).snow);

    river.tick(Tick::Snow);

    println!("{:?}", river.snow);
    println!("{:?}", river.borrow_child(1).snow);

    river.tick(Tick::FishHatch);
    println!("{:?}", river.borrow_child(0).salmon.len());

}
