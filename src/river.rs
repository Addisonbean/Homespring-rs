#[cfg(test)]
extern crate gag;

use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut, Ref};

use tick::Tick;
use salmon::{Salmon, Age, Direction};
use split_custom_escape::HomespringSplit;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    Other(String),
    Hatchery,
    HydroPower,
    Snowmelt,
    Shallows,
    Rapids,
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
            "shallows" => Shallows,
            "rapids" => Rapids,
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
pub struct Node<'a> {
    pub node_type: NodeType,
    pub parent: Weak<RefCell<Node<'a>>>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub salmon: Vec<Salmon<'a>>,
    pub powered: bool,
    pub watered: bool,
    pub snowy: bool,
    pub destroyed: bool,
}

impl<'a> Node<'a> { 
    pub fn new(name: &str) -> Node {
        let node = Node {
            node_type: NodeType::from_name(name),
            parent: Weak::new(),
            children: vec![],
            salmon: vec![],
            powered: false,
            watered: false,
            snowy: false,
            destroyed: false,
        };
        node.init()
    }

    fn init(mut self) -> Node<'a> {
        use self::NodeType::*;
        match &self.node_type {
            &Snowmelt => self.snowy = true,
            _ => (),
        }
        self
    }

    pub fn borrow_child(&self, n: usize) -> Ref<Node<'a>> {
        self.children[n].borrow()
    }

    pub fn borrow_mut_child(&self, n: usize) -> RefMut<Node<'a>> {
        self.children[n].borrow_mut()
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node<'a>>>) {
        self.children.push(child);
    }

    pub fn add_salmon(&mut self, salmon: Salmon<'a>) {
        self.salmon.push(salmon);
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
    pub fn run_tick(&mut self, tick: Tick) {
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
            (FishDown, _) => match Weak::upgrade(&self.parent) {
                Some(n) => (),
                None => {
                    for s in &self.salmon {
                        // write!(
                        print!("{}", s.name);
                    }
                    self.salmon = vec![];
                },
            },
            (FishHatch, &Hatchery) => {
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

    pub fn parse_program(code: &str) -> Rc<RefCell<Node>> {
        let mut tokens = HomespringSplit::new(code);

        let root_node = match tokens.next() {
            Some(name) => {
                Rc::new(RefCell::new(Node::new(name)))
            },
            None => unimplemented!(), // it's the quine thing
        };

        let mut current_node = Rc::clone(&root_node);

        for tok in tokens {
            if tok == "" {
                let parent = Weak::upgrade(&current_node.borrow().parent).unwrap();
                current_node = parent;
            } else {
                let child = Rc::new(RefCell::new(Node::new(tok)));
                child.borrow_mut().parent = Rc::downgrade(&current_node);
                current_node.borrow_mut().add_child(Rc::clone(&child));
                current_node = child;
            }
        }
        root_node
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

