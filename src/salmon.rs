#[derive(Debug, Eq, PartialEq)]
pub enum Age {
    Young,
    Mature,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Downstream,
    Upstream,
}

#[derive(Debug)]
pub struct Salmon<'a> {
    pub age: Age,
    pub direction: Direction,
    pub name: &'a str,
}
