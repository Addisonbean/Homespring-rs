extern crate homespring;

use homespring::river::Node;
use homespring::Tick;
use homespring::salmon::*;

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

    // ---

    let s = Salmon {
        age: Age::Young,
        direction: Direction::Downstream,
        name: "fishy fish\n",
    };
    let mut river = Node::new("universe");
    river.add_salmon(s);
    river.run_tick(Tick::FishDown);

}
