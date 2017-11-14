#[derive(Clone, Copy, Debug)]
pub enum Tick {
    Snow,
    Water,
    Power,
    FishDown,
    FishUp,
    FishHatch,
    Misc,
    Input,
}

#[derive(Clone, Copy, Debug)]
pub enum PropagationOrder {
    PreOrder,
    PostOrder,
    Any,
}

impl Tick {
    pub fn propagation_order(self) -> PropagationOrder {
        use self::PropagationOrder::*;
        use self::Tick::*;
        match self {
            /*
             * The documentation says that snow and water ticks are post order,
             * but the js implementation uses pre order. That made more sense
             * to me, given that using post order would cause the snow and water
             * to move downstream too quickly.
             * (https://github.com/quinkennedy/Homespring)
             */
            Snow => PreOrder,
            Water => PreOrder,
            Power => Any,
            FishDown => PreOrder,
            FishUp => PostOrder,
            FishHatch => PreOrder,
            Misc => PreOrder,
            Input => Any,
        }
    }

    // pub fn run(self, node: &mut RiverNode) {
        // println!("{:?} ticked", node.node_type);
    // }

    // pub fn run(self, node: RefMut<RiverNode>) {
        // use self::PropagationOrder::*;
        // match self.propagation_order() {
            // PostOrder => {
                // for i in 0..node.children.len() {
                    // self.run(borrow_mut_child(i));
                // }
                // println!("{:?} ticked", node.node_type);
            // },
            // PreOrder => {
                // println!("{:?} ticked", node.node_type);
                // for i in 0..node.children.len() {
                    // self.run(borrow_mut_child(i));
                // }
            // },
            // _ => unimplemented!(),
        // }

    // }
}

