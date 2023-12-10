#![allow(
    unused_parens,
    non_snake_case,
    dead_code,
    unused_variables,
    unreachable_code,
    unreachable_patterns,
    unused_imports
)]
use std::{arch::global_asm, char, collections::HashMap, fs, thread::current, usize};

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

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./src/inputs/day10.txt").unwrap();
    let vec = parseStrAsVec2D(&contents);
    let pipeMap = findLoop(&vec);
    let ghostRows: Vec<i32> = vec![];
    //  println!("{:?}", pipeMap);
    println!("{}", solvePart2(&vec, &pipeMap));
}

#[derive(Debug)]

enum Direction {
    South,
    East,
    North,
    West,
}

fn solvePart2(vec: &Vec<Vec<char>>, map: &HashMap<String, bool>) -> i32 {
    let mut amount = 0;
    let mut insideAmount = -1;
    let mut retS = "".to_owned();
    for y in 0..vec.len() {
        let mut inside = false;
        for x in 0..vec[y].len() {
            let t = vec[y as usize][x as usize];
            let pos = Pos2::new(x as i32, y as i32);
            match map.get(&pos.toString()) {
                None => {
                    if (inside) {
                        retS.push(t);
                        amount += 1;
                    } else {
                        retS.push('o');
                    }
                }
                Some(_) => {
                    if (t == '|' || t == 'F' || t == '7' || t == 'S') {
                        retS.push('o');
                        inside = !inside;
                    } else {
                        retS.push('o');
                    }
                }
            }
            // nicht: '-','J','L',
            // schon: 'F','7','|','S'
        }
        retS.push_str("\n");
    }
    fs::write("sol", retS);
    return amount;
}

fn pipeMapToString(vec: &Vec<Vec<char>>, map: &HashMap<String, bool>) -> String {
    let mut retString = "".to_owned();
    for y in 0..vec.len() {
        for x in 0..vec[y].len() {
            let p = Pos2::new(x as i32, y as i32);
            match map.get(&p.toString()) {
                Some(_) => {
                    let tile = vec[y][x];
                    retString.push(tile)
                }
                None => {
                    if (isTileOutsideLoop(vec, map, p)) {
                        retString.push('o');
                    } else {
                        retString.push('.');
                    }
                }
            }
        }
        retString.push_str("\n");
    }
    return retString;
}

fn isTileOutsideLoop(vec: &Vec<Vec<char>>, map: &HashMap<String, bool>, pos: Pos2) -> bool {
    let isOutside = false;
    let width = vec[0].len() as i32;
    let height = vec.len() as i32;
    let mut amountWalls = 0;
    for x in pos.x + 1..width {
        let newPos = Pos2::new(x, pos.y);
        match map.get(&newPos.toString()) {
            None => {}
            Some(_) => {
                amountWalls += 1;
                break;
            }
        }
    }
    for x in 0..pos.x {
        let newPos = Pos2::new(x, pos.y);
        match map.get(&newPos.toString()) {
            None => {}
            Some(_) => {
                amountWalls += 1;
                break;
            }
        }
    }
    for y in 0..pos.y {
        let newPos = Pos2::new(pos.x, y);
        match map.get(&newPos.toString()) {
            None => {}
            Some(_) => {
                amountWalls += 1;
                break;
            }
        }
    }
    for y in pos.y + 1..height {
        let newPos = Pos2::new(pos.x, y);
        match map.get(&newPos.toString()) {
            None => {}
            Some(_) => {
                amountWalls += 1;
                break;
            }
        }
    }
    return amountWalls < 4;
}
/*
fn isEnclosedByPipes(pos: Pos2, pipeMap: &HashMap<String, bool>, vec: &Vec<Vec<char>>) -> bool {
    let retVal = false;
    let width = vec[0].len() as i32;
    let height = vec.len() as i32;
    for x in pos.x + 1..width {
        let newPos = Pos2::new(x, pos.y);
        let tile = vec[pos.y as usize][x as usize];
        match pipeMap.get(&newPos.toString()){
            None => {}
            Some(val) => {
                if(tile == 'J' || tile == 'F' || tile == 'L' || tile == 'T' || tile == '7'){}
            }
        }
        if()
    }
    return retVal;
} */
fn findLoop(vec: &Vec<Vec<char>>) -> HashMap<String, bool> {
    let mut retMap: HashMap<String, bool> = HashMap::new();
    let startIndex = getIndexOfFirst('S', vec);
    let mut first = nextFromStart(startIndex, vec);
    let mut currentIndex = getCorrectIndex(startIndex, first, vec);
    let oldCurrent = currentIndex;

    retMap.insert(startIndex.toString(), true);
    retMap.insert(first.toString(), true);

    let mut steps = 1;
    while !startIndex.isEqual(currentIndex) {
        steps += 1;
        retMap.insert(currentIndex.toString(), true);
        let oldCurrent = currentIndex;
        currentIndex = getCorrectIndex(first, currentIndex, vec);
        first = oldCurrent;
    }
    println!("{}", (steps + 1) / 2);
    return retMap;
}

fn nextFromStart(startPos: Pos2, vec: &Vec<Vec<char>>) -> Pos2 {
    let curr = startPos;
    let newEast = Pos2::new(curr.x + 1, curr.y);
    let newWest = Pos2::new(curr.x - 1, curr.y);
    let newSouth = Pos2::new(curr.x + 0, curr.y + 1);
    let newNorth = Pos2::new(curr.x + 0, curr.y - 1);
    let eastChar = vec[newEast.y as usize][newEast.x as usize];
    let northChar = vec[newNorth.y as usize][newNorth.x as usize];
    let westChar = vec[newWest.y as usize][newWest.x as usize];
    let southChar = vec[newSouth.y as usize][newSouth.x as usize];

    if (northChar == '|' || northChar == '7' || northChar == 'F') {
        return newNorth;
    } else if (eastChar == '-' || eastChar == 'J' || eastChar == '7') {
        return newEast;
    } else if (westChar == 'L' || westChar == '-' || westChar == 'F') {
        return newWest;
    } else {
        return newSouth;
    }
}

fn getCorrectIndex(from: Pos2, curr: Pos2, vec: &Vec<Vec<char>>) -> Pos2 {
    let difference = Pos2::new(from.x - curr.x, from.y - curr.y);
    let lastDirection = if (difference.x == -1 && difference.y == 0) {
        Direction::West
    } else if (difference.x == 1 && difference.y == 0) {
        Direction::East
    } else if (difference.x == 0 && difference.y == -1) {
        Direction::North
    } else {
        Direction::South
    };
    let newEast = Pos2::new(curr.x + 1, curr.y);
    let newWest = Pos2::new(curr.x - 1, curr.y);
    let newSouth = Pos2::new(curr.x + 0, curr.y + 1);
    let newNorth = Pos2::new(curr.x + 0, curr.y - 1);
    let tile = vec[curr.y as usize][curr.x as usize];
    return match tile {
        '|' => match lastDirection {
            Direction::North => newSouth,
            _ => newNorth,
        },
        '-' => match lastDirection {
            Direction::East => newWest,
            _ => newEast,
        },
        'L' => match lastDirection {
            Direction::East => newNorth,
            _ => newEast,
        },
        'J' => match lastDirection {
            Direction::North => newWest,
            _ => newNorth,
        },
        '7' => match lastDirection {
            Direction::South => newWest,
            _ => newSouth,
        },
        'F' => match lastDirection {
            Direction::South => newEast,
            _ => newSouth,
        },
        _ => todo!(),
    };
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

fn getIndexOfFirst(needle: char, haystack: &Vec<Vec<char>>) -> Pos2 {
    let mut retVal = Pos2::new(0, 0);
    for y in 0..haystack.len() {
        for x in 0..haystack[y].len() {
            if (haystack[y][x] == needle) {
                retVal = Pos2::new(x as i32, y as i32);
            }
        }
    }
    return retVal;
}

fn getSurroundingElements(vec: &Vec<Vec<char>>, row: i32, column: i32) -> Vec<char> {
    let offsets = vec![
        [-1, -1],
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
    ];
    let mut retVec: Vec<char> = vec![];
    let width = vec[0].len() as i32;
    let height = vec.len() as i32;
    for e in offsets {
        let x = column as i32 + e[0] as i32;
        let y = row as i32 + e[1] as i32;
        if (x >= 0 && x < width && y >= 0 && y < height) {
            retVec.push(vec[x as usize][y as usize])
        }
    }
    return retVec;
}
