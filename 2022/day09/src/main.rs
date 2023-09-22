use std::{collections::HashSet, fs};

use crate::Movement::{Down, Left, Right, Up};

#[derive(Debug, Clone)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        return Self { x: 0, y: 0 };
    }
    fn coordinates(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
    fn goto(&mut self, movement: &Movement) -> (i32, i32) {
        match movement {
            Up => {
                self.y -= 1;
            }
            Down => {
                self.y += 1;
            }
            Left => {
                self.x -= 1;
            }
            Right => {
                self.x += 1;
            }
        }
        return (self.x, self.y);
    }
    fn follow(&mut self, head: Self) -> (i32, i32) {
        let delta_x = head.x - self.x;
        let delta_y = head.y - self.y;

        if delta_x.abs() > 2 || delta_y.abs() > 2 {
            panic!("Cannot follow the head. Head: {:?}, Tail: {:?}", head, self);
        }

        if delta_x == -2 {
            self.x -= 1;
            if delta_y.abs() == 1 {
                self.y += delta_y;
            }
        } else if delta_x == 2 {
            self.x += 1;
            if delta_y.abs() == 1 {
                self.y += delta_y;
            }
        }

        if delta_y == -2 {
            self.y -= 1;
            if delta_x.abs() == 1 {
                self.x += delta_x;
            }
        } else if delta_y == 2 {
            self.y += 1;
            if delta_x.abs() == 1 {
                self.x += delta_x;
            }
        }

        return (self.x, self.y);
    }
}

#[derive(Debug)]
struct Rope {
    points: Vec<Point>,
    tail_history: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(len: usize) -> Self {
        return Self {
            points: vec![Point::new(); len],
            tail_history: HashSet::new(),
        };
    }
    fn goto(&mut self, movement: Movement, count: u32) -> (i32, i32) {
        for _ in 0..count {
            self.points[0].goto(&movement);
            for i in 1..self.points.len() {
                let previous_point = self.points[i - 1].clone();
                self.points[i].follow(previous_point);
            }
            self.tail_history
                .insert(self.points[self.points.len() - 1].coordinates());
        }
        return (self.points[0].x, self.points[0].y);
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let movements: Vec<(Movement, u32)> = data
        .lines()
        .map(|x| {
            let mut line = x.split(' ');
            let movement: Movement = match line.next().unwrap() {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!("Failed to parse movement"),
            };
            let count: u32 = line.next().unwrap().parse().unwrap();

            return (movement, count);
        })
        .collect();

    // Part 1
    let mut rope = Rope::new(2);
    for command in movements.clone() {
        rope.goto(command.0, command.1);
    }
    println!("Part 1: {}", rope.tail_history.len());

    // Part 2
    let mut rope = Rope::new(10);
    for command in movements.clone() {
        rope.goto(command.0, command.1);
    }
    println!("Part 2: {}", rope.tail_history.len());
}
