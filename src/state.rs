use crate::state::point::Point;
use std::cmp::{Ordering, PartialEq};
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

pub mod point;
/// This is the definition of the state of a map as a struct
/// this contains location of cleaned places i.e. the places where vacuum cleaner has already
/// visited, uncleaned places, portals and moves that here moves are just nothing but a plan,
#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct State {
    pub start: Option<Point>,
    pub cleaned: Vec<Point>,
    pub uncleaned: Vec<Point>,
    pub portals: Vec<Point>,
    pub moves: Option<String>,
    pub check: bool,
    pub find: bool,
}

impl State {
    pub fn new(check: bool, find: bool) -> Self {
        State {
            start: None,
            cleaned: Vec::new(),
            uncleaned: Vec::new(),
            portals: Vec::new(),
            moves: None,
            check,
            find,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}\ncleaned: {},uncleaned: {}\nportals: {:?}\nmoves: {:?}\ncheck: {}, find: {}",
               self.start, self.cleaned.len(), self.uncleaned.len(), self.portals, self.moves, self.check, self.find)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.uncleaned.len().cmp(&self.uncleaned.len())
    }
}


impl State {

    /// this is the move cleaner method that simulates the movement of the cleaner.
    pub fn move_cleaner(&mut self, c: char) {
        if self.start != None {
            let point = self.start.unwrap().clone();
            if c == 'N' || c == 'S' {
                if point.x != 0usize && point.x != 11usize {
                    match c {
                        'N' => self.checker(Point {
                            x: point.x - 1usize,
                            y: point.y,
                        }),
                        'S' => self.checker(Point {
                            x: point.x + 1usize,
                            y: point.y,
                        }),
                        _ => (),
                    }
                }
            } else if c == 'W' || c == 'E' {
                if point.y != 0usize && point.y != 17usize {
                    match c {
                        'W' => self.checker(Point {
                            x: point.x,
                            y: point.y - 1usize,
                        }),
                        'E' => self.checker(Point {
                            x: point.x,
                            y: point.y + 1usize,
                        }),
                        _ => (),
                    }
                }
            }
        }
    }
    /// helper method for move_cleaner
    pub fn checker(&mut self, point: Point) {
        if self.portals.contains(&point) {
            if !self.portals.is_empty() {
                let mut m = Point {
                    x: 0usize,
                    y: 0usize,
                };
                for i in &self.portals {
                    if i != &point {
                        m.x = i.x;
                        m.y = i.y;
                    }
                }
                self.start = Some(m);
            }
        } else {
            if self.uncleaned.contains(&point) {
                self.start = Some(point);
                self.cleaned.push(point);
                self.uncleaned
                    .remove(self.uncleaned.binary_search(&point).unwrap());
            } else {
                if self.cleaned.contains(&point) {
                    self.start = Some(point);
                }
            }
        }
    }

    pub fn heuristics(&self) -> usize {
        self.uncleaned.len()
    }

    pub fn is_goal(&self) -> bool {
        self.uncleaned.is_empty()
    }

    pub fn get_neighbours(&self, mut s: String) -> Vec<(String, State)> {
        let mut result = Vec::new();
        let mut sol = Vec::new();
        for i in "WESN".chars() {
            let mut clone = self.clone();
            clone.move_cleaner(i);
            if !result.contains(&clone) && &clone != self {
                result.push(clone.clone());
                let mut y = s.clone();
                y.push(i);
                sol.push((y, clone));
            }
        }

        sol
    }
}
/// this is the main algorithm that runs greedy search on the state
/// here the algorithm finds valid plans.
impl State {
    pub fn find_plan(&mut self) {
        if self.moves == None {
            let mut map = BinaryHeap::new();
            map.push(Store::new(String::new(), self.clone()));
            let mut visited = HashSet::new();
            loop {
                let mut x = match map.pop() {
                    Some(Y) => Y,
                    None => break,
                };
                let Store { str: z, state: i } = x;
                if i.is_goal() {
                    self.moves = Some(z);
                    break;
                }
                for (s, t) in i.get_neighbours(z) {
                    if !visited.contains(&t) {
                        visited.insert(t.clone());
                        map.push(Store::new(s, t));
                    }
                }
            }
        }
    }
}
/// a helper struct to run the greedy search and store the plan in a string str: String,
#[derive(Eq, PartialEq)]
struct Store {
    str: String,
    state: State,
}

impl Store {
    pub fn new(str: String, state: State) -> Self {
        Store { str, state }
    }
}

impl PartialOrd for Store {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.state.cmp(&other.state))
    }
}

impl Ord for Store {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.cmp(&other.state)
    }
}
