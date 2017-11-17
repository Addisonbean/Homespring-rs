extern crate homespring;

use homespring::river::Node;
use homespring::Tick;

fn main() {

    let river = Node::parse_program("Universe hatchery  snowymelt");;
    let mut river = river.borrow_mut();

    println!("{:?}", river.snowy);
    println!("{:?}", river.borrow_child(1).snowy);

    river.tick(Tick::Snow);

    println!("{:?}", river.snowy);
    println!("{:?}", river.borrow_child(1).snowy);

    river.tick(Tick::FishHatch);
    println!("{:?}", river.borrow_child(0).salmon.len());

}
