#![feature(test)]
extern crate test;
use std::collections::HashMap;
use std::fs::read_to_string;

mod intcode;
mod point;

use intcode::*;
use point::*;

#[derive(Debug, Clone)]
struct Bot {
    pos: Point,
    facing: Point,
    map: HashMap<Point, i64>,
}

impl Bot {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            facing: Point { x: 0, y: -1 },
            map: HashMap::new(),
        }
    }

    pub fn run(&mut self, vm: &mut IntCodeVM, init: i64) -> usize {
        let mut output = vec![];
        let mut out = 0;
        self.map.insert(self.pos, init);
        'a: loop {
            let tile = self.map.entry(self.pos).or_insert(0);
            let input = *tile;
            if vm.execute(input, &mut output) {
                break 'a;
            }
            out += 1;
            if out == 2 {
                *tile = output[0];
                let dir = match output[1] {
                    0 => Direction::CounterClock,
                    1 => Direction::Clockwise,
                    _ => unreachable!(),
                };
                self.facing = self.facing.rotated_90(dir);
                self.pos = self.pos + self.facing;
                out = 0;
                output.clear();
            }
        }
        self.map.len()
    }
}

fn main() {
    let program = read_to_string("input.txt").unwrap();
    let mut vm = parse_program(&program);
    let mut bot = Bot::new(Point { x: 0, y: 0 });
    let res = bot.run(&mut vm.clone(), 0);
    println!("Part1: {}", res);

    let mut bot = Bot::new(Point { x: 0, y: 0 });
    bot.run(&mut vm, 1);
    let map = bot
        .map
        .iter()
        .filter_map(|(p, val)| if *val == 1 { Some(p.clone()) } else { None })
        .collect::<Vec<_>>();
    let mut min = Point::default();
    let mut max = Point::default();
    for p in map.iter() {
        if p.x < min.x {
            min.x = p.x;
        }
        if p.y < min.y {
            min.y = p.y;
        }
        if p.x > max.x {
            max.x = p.x;
        }
        if p.y > max.y {
            max.y = p.y;
        }
    }
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let mut out = " ";
            if let Some(v) = bot.map.get(&Point { x, y }) {
                if *v == 1 {
                    out = "#";
                }
            }
            print!("{}", out);
        }
        print!("\n");
    }
}
