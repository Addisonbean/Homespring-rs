use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut, Ref};

use tick::Tick;
use salmon::{Salmon, Age, Direction};
use split_custom_escape::HomespringSplit;

#[derive(Debug)]
pub enum RiverNodeType {
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

impl RiverNodeType {
    pub fn from_name(name: &str) -> RiverNodeType {
        // unimplemented!();
        use self::RiverNodeType::*;
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
pub struct RiverNode<'a> {
    pub node_type: RiverNodeType,
    pub parent: Weak<RefCell<RiverNode<'a>>>,
    pub children: Vec<Rc<RefCell<RiverNode<'a>>>>,
    pub salmon: Vec<Salmon<'a>>,
    pub power: bool,
    pub water: bool,
    pub snow: bool,
    pub destroyed: bool,
}

impl<'a> RiverNode<'a> { 
    pub fn new(name: &str) -> RiverNode {
        let node = RiverNode {
            node_type: RiverNodeType::from_name(name),
            parent: Weak::new(),
            children: vec![],
            salmon: vec![],
            power: false,
            water: false,
            snow: false,
            destroyed: false,
        };
        node.init()
    }

    pub fn init(mut self) -> RiverNode<'a> {
        use self::RiverNodeType::*;
        match &self.node_type {
            &Snowmelt => self.snow = true,
            _ => (),
        }
        self
    }

    pub fn borrow_child(&self, n: usize) -> Ref<RiverNode<'a>> {
        self.children[n].borrow()
    }

    pub fn borrow_mut_child(&self, n: usize) -> RefMut<RiverNode<'a>> {
        self.children[n].borrow_mut()
    }

    pub fn add_child(&mut self, child: Rc<RefCell<RiverNode<'a>>>) {
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

    // I don't like this inside of RiverNode...
    pub fn run_tick(&mut self, tick: Tick) {
        use self::RiverNodeType::*;
        use tick::Tick::*;
        match (tick, &self.node_type) {
            (Snow, _) => {
                for i in 0..self.children.len() {
                    if self.borrow_child(i).snow {
                        self.snow = true;
                        break;
                    }
                }
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

    pub fn parse_program(code: &str) -> Rc<RefCell<RiverNode>> {
        let mut tokens = HomespringSplit::new(code);

        let root_node = match tokens.next() {
            Some(name) => {
                Rc::new(RefCell::new(RiverNode::new(name)))
            },
            None => unimplemented!(), // it's the quine thing
        };

        let mut current_node = Rc::clone(&root_node);

        for tok in tokens {
            if tok == "" {
                let parent = Weak::upgrade(&current_node.borrow().parent).unwrap();
                current_node = parent;
            } else {
                let child = Rc::new(RefCell::new(RiverNode::new(tok)));
                child.borrow_mut().parent = Rc::downgrade(&current_node);
                current_node.borrow_mut().add_child(Rc::clone(&child));
                current_node = child;
            }
        }
        root_node
    }
}

