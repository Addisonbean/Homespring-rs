#[cfg(test)]
extern crate gag;

use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut, Ref};

use tick::Tick;
use salmon::{Salmon, Age, Direction};
use split_custom_escape::HomespringSplit;
use program::Program;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    Other(String),
    Hatchery,
    HydroPower,
    Snowmelt,
    Shallows(u8),
    Rapids(u8),
    AppendDown,
    Bear,
    ForceField,
    Sense,
    Clone,
    YoungBear,
    Bird,
    UpstreamKillingDevice,
    Waterfall,
    Universe,
    Powers,
    Marshy,
    Insulted,
    UpstreamSense,
    DownstreamSense,
    Evaporates,
    YouthFountain,
    Oblivion,
    Pump,
    RangeSense,
    Fear,
    ReverseUp,
    ReverseDown,
    Time,
    Lock,
    InverseLock,
    YoungSense,
    Switch,
    YoungSwitch,
    Narrows,
    AppendUp,
    YoungRangeSense,
    Net,
    ForceDown,
    ForceUp,
    Spawn,
    PowerInvert,
    Current,
    Bridge,
    Split,
    RangeSwitch,
    YoungRangeSwitch,
}

impl NodeType {
    pub fn from_name(name: &str) -> NodeType {
        // unimplemented!();
        use self::NodeType::*;
        match &name.to_lowercase()[..] {
            "hatchery" => Hatchery,
            "hydro. power" => HydroPower,
            "snowmelt" => Snowmelt,
            "shallows" => Shallows(2),
            "rapids" => Rapids(2),
            "append. down" => AppendDown,
            "bear" => Bear,
            "force. field" => ForceField,
            "sense" => Sense,
            "clone" => Clone,
            "young bear" => YoungBear,
            "bird" => Bird,
            "upstream. killing. device" => UpstreamKillingDevice,
            "waterfall" => Waterfall,
            "universe" => Universe,
            "powers" => Powers,
            "marshy" => Marshy,
            "insulated" => Insulted,
            "upstream. sense" => UpstreamSense,
            "downstream. sense" => DownstreamSense,
            "evaporates" => Evaporates,
            "youth. fountain" => YouthFountain,
            "oblivion" => Oblivion,
            "pump" => Pump,
            "range. sense" => RangeSense,
            "fear" => Fear,
            "reverse. up" => ReverseUp,
            "reverse. down" => ReverseDown,
            "time" => Time,
            "lock" => Lock,
            "inverse. lock" => InverseLock,
            "young. sense" => YoungSense,
            "switch" => Switch,
            "young. switch" => YoungSwitch,
            "narrows" => Narrows,
            "append. up" => AppendUp,
            "young. range. sense" => YoungRangeSense,
            "net" => Net,
            "force. down" => ForceDown,
            "force. up" => ForceUp,
            "spawn" => Spawn,
            "power. invert" => PowerInvert,
            "current" => Current,
            "bridge" => Bridge,
            "split" => Split,
            "range. switch" => RangeSwitch,
            "young. range. switch" => YoungRangeSwitch,
            _ => Other(name.to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct Node<'a, 'b> {
    pub name: &'b str,
    pub node_type: NodeType,
    pub parent: Weak<RefCell<Node<'a, 'b>>>,
    pub children: Vec<Rc<RefCell<Node<'a, 'b>>>>,
    pub salmon: Vec<Salmon<'a>>,
    pub block_salmon: bool,
    pub very_block_salmon: bool,
    pub powered: bool,
    pub block_power: bool,
    pub watered: bool,
    pub block_water: bool,
    pub snowy: bool,
    pub block_snow: bool,
    pub destroyed: bool,
}

impl<'a, 'b> Node<'a, 'b> { 
    pub fn new(name: &'b str) -> Node<'a, 'b> {
        let node = Node {
            name,
            node_type: NodeType::from_name(name),
            parent: Weak::new(),
            children: vec![],
            salmon: vec![],
            block_salmon: false,
            very_block_salmon: false,
            powered: false,
            block_power: false,
            watered: false,
            block_water: false,
            snowy: false,
            block_snow: false,
            destroyed: false,
        };
        node.init()
    }

    fn init(mut self) -> Node<'a, 'b> {
        use self::NodeType::*;
        match &self.node_type {
            &Snowmelt => self.snowy = true,
            &Powers => self.powered = true,
            _ => (),
        }
        self
    }

    pub fn borrow_child(&self, n: usize) -> Ref<Node<'a, 'b>> {
        self.children[n].borrow()
    }

    pub fn borrow_mut_child(&self, n: usize) -> RefMut<Node<'a, 'b>> {
        self.children[n].borrow_mut()
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node<'a, 'b>>>) {
        self.children.push(child);
    }

    pub fn add_salmon(&mut self, salmon: Salmon<'a>) {
        self.salmon.push(salmon);
    }

    // Returns the index of the child that would lead to the node
    // with a name of `name`.
    pub fn find_node_path(&self, name: &str) -> Option<usize> {
        (0..self.children.len()).position(|i|
            self.borrow_child(i).find_node(name)
        )
    }

    // This is supposed to use an in-order search, but that doesn't
    // really make sense for an n-ary tree...
    // This will at least be in-order for any nodes with <= 2 children.
    fn find_node(&self, name: &str) -> bool {
        let len = self.children.len();
        if len > 0 {
            match self.borrow_child(0).find_node(name) {
                true => return true,
                false => (),
            }
        }
        if self.name == name { return true; }
        if len > 1 {
            for i in 1..len {
                match self.borrow_child(i).find_node(name) {
                    true => return true,
                    false => (),
                }
            }
        }
        false
    }

    // something to move fish up and down stream
    pub fn move_salmon(&mut self, direction: Direction) {
        match &mut self.node_type {
            &mut NodeType::Shallows(ref mut i) =>
                if *i > 0 {
                    *i -= 1;
                    return
                },
            &mut NodeType::Rapids(ref mut i) => 
                if *i > 0 {
                    *i -= 1;
                    return
                },
            _ => (),
        }
        match direction {
            Direction::Downstream => {
                match self.parent.upgrade() {
                    Some(p) => {
                        // Use `Vec::drain_filter` when once it stabilizes: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
                        let mut p = p.borrow_mut();
                        let mut i = 0;
                        while i != self.salmon.len() {
                            if self.salmon[i].direction == Direction::Downstream {
                                let s = self.salmon.remove(i);
                                p.salmon.push(s);
                            } else {
                                i += 1;
                            }
                        }
                    },
                    None => {
                        for s in &self.salmon {
                            if s.direction == Direction::Downstream {
                                print!("{}", s.name);
                            }
                        }
                        self.salmon.retain(|s| s.direction != Direction::Downstream);
                    },
                }
            },
            Direction::Upstream => {
                if self.block_salmon { return }

                // `Vec::drain_filter` could probably be used here too
                let mut i = 0;
                while i != self.salmon.len() {
                    if self.salmon[i].direction == Direction::Upstream {
                        let idx = match self.find_node_path(self.salmon[i].name) {
                            Some(idx) if !self.borrow_child(idx).very_block_salmon
                                => Some(idx),
                            _ => self.children.iter().position(|c| !c.borrow().very_block_salmon),
                        };
                        match idx {
                            Some(idx) => {
                                let s = self.salmon.remove(i);
                                self.borrow_mut_child(idx).salmon.push(s);
                            },
                            None => i += 1,
                        }
                    } else {
                        i += 1;
                    }
                }
            },
        }
    }

    pub fn tick(&mut self, tick: Tick) {
        use tick::PropagationOrder::*;
        match tick.propagation_order() {
            PostOrder => {
                for i in 0..self.children.len() {
                    self.borrow_mut_child(i).tick(tick);
                }
                self.run_tick(tick);
            },
            PreOrder => {
                self.run_tick(tick);
                for i in 0..self.children.len() {
                    self.borrow_mut_child(i).tick(tick);
                }
            },
            _ => unimplemented!(),
        }
    }

    // TODO: rewrite this, it's crap
    // I don't like this inside of Node... (or do I...?)
    fn run_tick(&mut self, tick: Tick) {
        use self::NodeType::*;
        use tick::Tick::*;
        match (tick, &self.node_type) {
            (Snow, _) => {
                for i in 0..self.children.len() {
                    if self.borrow_child(i).snowy {
                        self.become_snowy();
                        break;
                    }
                }
            },
            (Water, _) => {
                for i in 0..self.children.len() {
                    if self.borrow_child(i).watered {
                        self.become_watered();
                        break;
                    }
                }
            },
            (Power, &HydroPower) => self.powered = self.watered,
            (FishDown, _) => self.move_salmon(Direction::Downstream),
            (FishUp, _) => self.move_salmon(Direction::Upstream),
            (FishHatch, &Hatchery) => if self.is_powered() {
                self.add_salmon(Salmon {
                    age: Age::Mature,
                    direction: Direction::Upstream,
                    name: "homeless"
                });
            },
            _ => (),
        }
    }

    // TODO: I don't like this...
    pub fn become_snowy(&mut self) {
        use self::NodeType::*;
        self.snowy = true;
        match self.node_type {
            HydroPower => self.destroyed = true,
            _ => (),
        }
    }

    pub fn become_watered(&mut self) {
        self.watered = true;
    }

    pub fn is_powered(&self) -> bool {
        if self.block_power {
            false
        } else if self.powered {
            true
        } else {
            self.children.iter().any(|c| {
                c.borrow_mut().is_powered()
            })
        }
    }

    pub fn parse_program(code: &str) -> Program {
        let mut tokens = HomespringSplit::new(code);

        let root_node = match tokens.next() {
            Some(name) => {
                Rc::new(RefCell::new(Node::new(name)))
            },
            None => return Program::Quine,
        };

        let mut current_node = Rc::clone(&root_node);

        for tok in tokens {
            if tok == "" {
                let parent = current_node.borrow().parent.upgrade().unwrap();
                current_node = parent;
            } else {
                let child = Rc::new(RefCell::new(Node::new(tok)));
                child.borrow_mut().parent = Rc::downgrade(&current_node);
                current_node.borrow_mut().add_child(Rc::clone(&child));
                current_node = child;
            }
        }
        Program::River(root_node)
    }
}

// #[test]
// fn print_salmon_name() {
    // use std::io::Read;
    // use self::gag::BufferRedirect;
    // let name = "fishy fish";
    // let s = Salmon {
        // age: Age::Young,
        // direction: Direction::Downstream,
        // name,
    // };
    // let mut river = Node::new("universe");
    // river.add_salmon(s);

    // let mut out = String::new();
    // let mut buf = BufferRedirect::stdout().unwrap();

    // river.run_tick(Tick::FishDown);
    // buf.read_to_string(&mut out);

    // assert_eq!(0, river.salmon.len());
    // assert_eq!(&out[..], name);
// }

