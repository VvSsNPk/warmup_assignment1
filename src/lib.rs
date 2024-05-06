use std::any::Any;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::{File, FileType};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use crate::state::point::Point;
use crate::state::State;

pub mod state;

pub fn create_state(path: &PathBuf) -> State{
    let mut f = File::open(path);
    let mut state =State::new(false,false);
    let reader = BufReader::new(f.unwrap());
    let mut m = 0usize;
    let mut p = 0usize;
    for line in reader.lines(){
        let mut n = 0usize;
        let mut store = String::from(line.unwrap());
        if store.contains("X"){
            for c in store.chars(){
                match c{
                    'S' => {state.start = Some(Point::new(m, n));
                        state.cleaned.push(Point::new(m,n));},
                    ' ' => {state.uncleaned.push(Point::new(m,n));},
                    'P' => {state.portals.push(Point::new(m,n));},
                    _ => (),
                }
                n = n + 1usize;
            }
            m = m + 1usize;
        }else {
            if store.trim() != "FIND PLAN" && store.trim() != "CHECK PLAN"{
                state.moves = Some(store);
            }else{
                if store.trim() == "FIND PLAN"{
                    state.find = true;
                }else{
                    state.check = true;
                }
            }
        }
    }


    state
}

pub fn process_state_start_given(state: &mut State){
    if state.check{
        if state.moves != None{
            let str = state.moves.clone().unwrap();
            for i in str.chars(){
                state.move_cleaner(i);
            }
        }
    }else{
        state.find_plan();
    }
}

pub fn process_state_start_not_given(st: &mut State) -> HashSet<Point>{
    let mut result = HashSet::new();
    let unclean = st.uncleaned.clone();
    for i in unclean{
        let mut state = st.clone();
        state.start = Some(i.clone());
        state.uncleaned.remove(state.uncleaned.binary_search(&i).unwrap());
        state.cleaned.push(i);
        let str = state.moves.clone().unwrap();
        for i in str.chars() {
            state.move_cleaner(i);
        }
        result.extend(state.uncleaned.clone());
    }


    result
}


pub fn write_to_file_start_given(state: State, path: &mut PathBuf,filename: &str) -> std::io::Result<()>{
    create_dir_sol(path);
    path.push(filename);
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    if state.check{
        if state.uncleaned.is_empty(){
            writeln!(&mut buffer,"GOOD PLAN")?;
        }else{
            writeln!(&mut buffer,"BAD PLAN")?;
            for i in state.uncleaned.clone(){
                writeln!(&mut buffer,"{}, {}",i.y,i.x)?;
            }
        }
    }else{
        writeln!(&mut buffer,"{}",state.moves.unwrap())?;
    }
    Ok(())
}

pub fn write_to_file_start_not_given(set:HashSet<Point>, path :&mut PathBuf, filename: &str) -> std::io::Result<()>{
    create_dir_sol(path);
    path.push(filename);
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    if set.is_empty(){
        writeln!(&mut buffer,"GOOD PLAN")?;
    }else{
        writeln!(&mut buffer,"BAD PLAN")?;
        for i in set{
            writeln!(&mut buffer,"{}, {}",i.y,i.x)?;
        }
    }

    Ok(())
}

pub fn write_to_file_start_not_given_find(state: &mut State,path: &mut PathBuf,filename: &str) -> std::io::Result<()>{
    create_dir_sol(path);
    path.push(filename);
    let f = File::create(path)?;
    let mut buffer = BufWriter::new(f);
    if state.start==None && state.find{
        let mut st = ElevateMap::create(state);
        let mut el = st.unwrap();
        let str= el.find_plan();
        writeln!(&mut buffer,"{}",str)?;
    }


    Ok(())

}

fn create_dir_sol(path: &mut PathBuf){
    path.push("solutions");
    if !path.exists(){
        fs::create_dir(path).unwrap();
    }
}

pub fn directory_parser(path: &mut PathBuf){
    if path.is_dir(){
        for contents in path.read_dir().expect("Cannot read directory"){
            let mut p = contents.unwrap().path();
            if p.is_file(){
                let mut state = create_state(&mut p);
                let file_name = &p.file_name().unwrap().clone().to_str().unwrap().replace("problem","solution");
                if state.start != None{
                    process_state_start_given(&mut state);
                    write_to_file_start_given(state, &mut path.clone(),file_name).unwrap()
                }else{
                    if state.check {
                        let set = process_state_start_not_given(&mut state);
                        write_to_file_start_not_given(set, &mut path.clone(), file_name).unwrap()
                    }else{
                        write_to_file_start_not_given_find(&mut state,&mut path.clone(),file_name).unwrap()
                    }
                }
            }
        }
    }
}

#[derive(Clone,Eq, PartialEq,Hash)]
pub struct ElevateMap{
    pub map: Vec<Speicher>,
    pub state: State,
}

impl ElevateMap{
    fn new() -> Self{
        ElevateMap{
            map : Vec::new(),
            state: State::new(false,true),

        }
    }

    pub fn create(state: &mut State) -> Option<Self>{
        if state.start == None {
            let mut map = ElevateMap::new();
            let mut uncleaned = state.uncleaned.clone();
            map.state = state.clone();
            for i in uncleaned{
                let mut speicher = Speicher::new(i.x, i.y);
                let mut uncleaned = state.uncleaned.clone();
                uncleaned.remove(state.uncleaned.binary_search(&i).unwrap());
                speicher.uncleaned.extend(uncleaned);
                map.map.push(speicher);
            }
            return Some(map)
        }else {
            None
        }
    }

    pub fn move_cleaner(&mut self, c :char){
        let portals = self.state.portals.clone();
        for  i in &mut self.map{
            let mut point = i.start.clone();
            if point.x != 0usize && point.x != 11usize && point.y != 0usize && point.y != 17usize {
                match c {
                    'N' => point.x = point.x - 1usize,
                    'S' => point.x = point.x + 1usize,
                    'E' => point.y = point.y + 1usize,
                    'W' => point.y = point.y - 1usize,
                    _ => (),
                }
                i.change_start(point,&portals,&self.state.uncleaned);
            }

        }
    }


    fn is_goal(&self,s:&String)-> bool{
        if self.state.start ==None{
            let mut uncleaned = self.state.uncleaned.clone();
            for i in uncleaned{
                let mut state = self.state.clone();
                state.start = Some(i);
                for j in s.clone().chars(){
                    state.move_cleaner(j);
                }
                if !state.uncleaned.is_empty(){
                    return false;
                }
            }
            return true
        }else {
            false
        }
    }

    fn get_neighbours(&self,mut s:String) -> Vec<(String,ElevateMap)>{
        let mut result = Vec::new();
        for i in "NEWS".chars(){
            let mut y = s.clone();
            let mut x = self.clone();
            x.move_cleaner(i);
            if x!= *self {
                y.push(i);
                result.push((y, x));
            }


        }
        result
    }

    pub fn find_plan(&mut self) -> String{
        let mut frontier = BinaryHeap::new();
        frontier.push(Pair::new(String::new(),self.clone()));
        let mut visited = HashSet::new();
        loop {
            let mut x = match frontier.pop(){
                Some(y) => y,
                None => break,
            };
            let Pair{str,map} =x;
            //println!("{}",str);
            if map.is_goal(&str){
                return str;
            }
            for (f,k) in map.get_neighbours(str){
                if !visited.contains(&k){
                    visited.insert(k.clone());
                    frontier.push(Pair::new(f,k));
                }
            }
        }



        String::new()
    }
}

impl Ord for ElevateMap{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut count = 0;
        let mut count2 = 0;
        for i in &self.map{
            count += i.uncleaned.len();
        }
        for i in &other.map{
            count2 += i.uncleaned.len();
        }
        count.cmp(&count2)
    }
}

impl PartialOrd for ElevateMap{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(&self))
    }
}

impl Display for ElevateMap{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{}",self.map.len(),self.state)
    }
}


#[derive(Clone,Ord, PartialOrd, Eq, PartialEq,Hash)]
pub struct Speicher{
    pub start: Point,
    pub uncleaned: Vec<Point>,
}
impl Display for Speicher{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{:?}",self.start,self.uncleaned)
    }
}
impl Speicher{
    pub fn new(x: usize,y:usize) -> Self{
        Speicher{
            start:Point::new(x,y),
            uncleaned: Vec::new(),
        }
    }

    pub fn change_start(&mut self, point: Point,portals:& Vec<Point>,uncleaned:& Vec<Point>){
        if portals.contains(&point){
            for i in portals{
                if i != &point{
                    self.start = i.clone();
                }
            }
        }else if uncleaned.contains(&point){
            self.start =point;
            if self.uncleaned.contains(&point){
                self.uncleaned.remove(self.uncleaned.binary_search(&point).unwrap());
            }
        }
    }
}
#[derive(Eq, PartialEq,Hash)]
struct Pair{
    str: String,
    map :ElevateMap,
}

impl Pair{
    pub fn new(str:String,map:ElevateMap) -> Self{
        Pair{
            str,
            map,
        }
    }
}

impl PartialOrd for Pair{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.map.cmp(&self.map))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.map.cmp(&self.map)
    }
}