use std::rc::Rc;
use std::cell::RefCell;

use river::Node;

pub enum Program<'a, 'b> {
    Quine,
    River(Rc<RefCell<Node<'a, 'b>>>),
}

impl<'a, 'b> Program<'a, 'b> {
    pub fn execute(&mut self) {
        use self::Program::*;
        match self {
            &mut Quine => println!("In Homespring, the null program is not a quine."),
            &mut River(ref mut n) => unimplemented!(),
        }
    }

    // maybe only include this in tests?
    pub fn to_node(self) -> Option<Rc<RefCell<Node<'a, 'b>>>> {
        match self {
            Program::River(n) => Some(n),
            _ => None,
        }
    }
}

