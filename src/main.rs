#![allow(
    unused_parens,
    non_snake_case,
    dead_code,
    unused_variables,
    unreachable_code,
    unreachable_patterns,
    unused_imports
)]

use std::{collections::HashMap, fs, process::id};

#[derive(Debug, Clone, Copy)]
struct Pos2 {
    x: i32,
    y: i32,
}

impl Pos2 {
    pub fn new(x: i32, y: i32) -> Pos2 {
        return Pos2 { x, y };
    }

    pub fn isEqual(&self, other: Pos2) -> bool {
        return other.x == self.x && self.y == other.y;
    }

    pub fn toString(&self) -> String {
        return self.x.to_string() + "," + &self.y.to_string();
    }
}

type Observation = Vec<Vec<char>>;
type GalaxyMap = Vec<Pos2>;
fn main() {
    let contents = fs::read_to_string("./src/inputs/day11.txt").unwrap();
    let observation = parseStrAsVec2D(&contents);
    let expanded = expandUniverse(&observation);
    //let map = parseMapToString(&expanded)
    let map = cratePosMap(&expanded);

    println!("map{:?}", galaxyDist(&map) / 2);
    //printStr(&map);
}

fn distanceBetweenPoints(p1: &Pos2, p2: &Pos2) -> i32 {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
}

fn galaxyDist(map: &GalaxyMap) -> i32 {
    let mut sum = 0;
    for x in map {
        for y in map {
            sum += distanceBetweenPoints(&x, &y);
        }
    }
    return sum;
}

fn printStr(str: &str) {
    for x in str.chars() {
        if x == '\n' {
            println!()
        } else {
            print!("{}", x);
        }
    }
}

fn parseStrAsVec2D(str: &str) -> Vec<Vec<char>> {
    let mut outerVec: Vec<Vec<char>> = vec![];
    for x in str.split("\n") {
        let mut innerVec: Vec<char> = vec![];
        for y in x.chars() {
            innerVec.push(y);
        }
        outerVec.push(innerVec);
    }
    return outerVec;
}

fn parseMapToString(vec: &Vec<Vec<char>>) -> String {
    let mut retString = "".to_owned();
    for y in 0..vec.len() {
        for x in 0..vec[y].len() {
            let tile = vec[y][x];
            retString.push(tile);
        }
        retString.push_str("\n");
    }
    return retString;
}

fn doesRowContainX(needle: char, hayStack: &Vec<char>) -> bool {
    let mut found = false;
    for x in hayStack {
        if (*x == needle) {
            found = true;
            break;
        }
    }
    return found;
}

fn getColumn(vec: &Observation, idx: usize) -> Vec<char> {
    let mut retVec: Vec<char> = vec![];
    for x in vec {
        retVec.push(x[idx]);
    }
    return retVec;
}

fn cratePosMap(vec: &Observation) -> GalaxyMap {
    let mut map: GalaxyMap = vec![];
    for y in 0..vec.len() {
        for x in 0..vec[y].len() {
            if (vec[y][x] == '#') {
                map.push(Pos2::new(x as i32, y as i32));
            }
        }
    }
    return map;
}

fn pushColumn(vec: &Observation, column: Vec<char>) -> Observation {
    let mut retVec = vec.clone();
    if (vec.len() == 0) {
        for i in 0..column.len() {
            retVec.push(vec![column[i]]);
        }
        return retVec;
    }
    for i in 0..column.len() {
        retVec[i].push(column[i]);
    }
    return retVec;
}
