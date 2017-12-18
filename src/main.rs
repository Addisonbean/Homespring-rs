extern crate homespring;

use homespring::river::Node;
use homespring::Tick;
use homespring::salmon::*;
use homespring::Program;

fn main() {

    let river = Node::parse_program("Universe hatchery powers   snowmelt").to_node().unwrap();
    let mut river = river.borrow_mut();

    println!("{:?}", river.snowy);
    println!("{:?}", river.borrow_child(1).snowy);

    river.tick(Tick::Snow);

    println!("{:?}", river.snowy);
    println!("{:?}", river.borrow_child(1).snowy);

    river.tick(Tick::FishHatch);
    println!("{:?}", river.borrow_child(0).salmon.len());

    println!("{:?}", river.find_node_path("hatchery"));
    println!("{:?}", river.find_node_path("powers"));
    println!("{:?}", river.find_node_path("snowmelt"));
    println!("{:?}", river.find_node_path("nope"));

    river.tick(Tick::FishUp);

    println!("{:?}", river.borrow_child(0).salmon.len());
    let kiddo = river.borrow_child(0);
    println!("{:?}", kiddo.borrow_child(0).salmon.len());

    // ---

    let s = Salmon {
        age: Age::Young,
        direction: Direction::Downstream,
        name: "fishy fish\n",
    };
    let mut river = Node::new("universe");
    river.add_salmon(s);
    river.tick(Tick::FishDown);

    // ---
    
    let mut p = Program::Quine;
    p.execute();

    // ---

    // let river = Node::parse_program("Universe hatchery powers   snowymelt").to_node().unwrap();

}
