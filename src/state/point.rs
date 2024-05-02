use std::fmt::{Display, Formatter};


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug,Ord, PartialOrd)]
pub struct Point{
    pub x: usize,
    pub y: usize,
}
impl Point{
    pub fn new(x:usize, y: usize) -> Self{
        Point{
            x,
            y,
        }
    }
}

impl  Display for Point{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}",self.x, self.y)
    }
}
