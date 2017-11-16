#[derive(Debug)]
pub enum Age {
    Young,
    Mature,
}

#[derive(Debug)]
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
