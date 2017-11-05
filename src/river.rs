use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
        match name {
            _ => Other(name.to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct RiverNode {
    pub node_type: RiverNodeType,
    pub parent: Weak<RefCell<RiverNode>>,
    pub children: Vec<Rc<RefCell<RiverNode>>>,
    pub power: bool,
    pub water: bool,
    pub snow: bool,
    pub destroyed: bool,
}

impl RiverNode { 
    pub fn new(name: &str) -> RiverNode {
        RiverNode {
            node_type: RiverNodeType::from_name(name),
            parent: Weak::new(),
            children: vec![],
            // TODO: these defaults may be wrong, for example when the name is "snowmelt" snow
            // should be true
            power: false,
            water: false,
            snow: false,
            destroyed: false,
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<RiverNode>>) {
        self.children.push(child);
    }

    pub fn parse_program(code: &str) -> Rc<RefCell<RiverNode>> {
        let mut tokens = code.split(' ');

        let root_node = match tokens.next() {
            Some(name) => {
                Rc::new(RefCell::new(RiverNode::new(name)))
            },
            None => panic!(), // it's the quine thing
        };

        let mut current_node = Rc::clone(&root_node);

        for tok in tokens {
            if tok == "" {
            } else {
                let child = Rc::new(RefCell::new(RiverNode::new(tok)));
                {
                    child.borrow_mut().parent = Rc::downgrade(&current_node);
                }
                current_node.borrow_mut().add_child(Rc::clone(&child));
                current_node = child;
            }
        }
        root_node
    }
}

// http://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
// https://cs.stackexchange.com/questions/44820/what-does-pre-post-and-in-order-walk-mean-for-a-n-ary-tree

