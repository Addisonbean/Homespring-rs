extern crate homespring;

use homespring::RiverNode;
use homespring::Tick;

fn main() {

    let river = RiverNode::parse_program("Universe bear snowmelt");
    let mut river = river.borrow_mut();
    river.tick(Tick::Snow);

}
