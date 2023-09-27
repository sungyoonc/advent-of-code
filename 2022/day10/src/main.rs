use std::fs;

#[derive(Debug)]
enum InstructionSet {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Register {
    x: i32,
}

#[derive(Debug)]
struct Cpu {
    cycle: i32,
    register: Register,
    log_points: Vec<i32>,
    log: Vec<i32>,
    crt: Crt,
}

impl Cpu {
    fn new_with_log_points(log_points: &Vec<i32>) -> Self {
        Self {
            cycle: 0,
            register: Register { x: 1 },
            log_points: log_points.clone(),
            log: Vec::new(),
            crt: Crt::new(),
        }
    }
    fn add_cycle(&mut self) {
        self.cycle += 1;

        // Log current register
        if self.log_points.contains(&self.cycle) {
            self.log.push(self.register.x);
        }

        // Draw pixel
        self.crt.draw_next_pixel(self.register.x);
    }
    fn run(&mut self, instruction: InstructionSet) {
        match instruction {
            InstructionSet::Addx(v) => {
                self.add_cycle();
                self.add_cycle();
                self.register.x += v;
            }
            InstructionSet::Noop => {
                self.add_cycle();
            }
        }
    }
}

#[derive(Debug)]
struct Crt {
    current_pixel: (i32, i32),
    screen: String,
}

impl Crt {
    fn new() -> Self {
        Crt {
            current_pixel: (0, 0),
            screen: "".into(),
        }
    }
    fn draw_next_pixel(&mut self, sprite_pos: i32) {
        let pixel: &str;

        if (self.current_pixel.0 >= (sprite_pos - 1)) && (self.current_pixel.0 <= (sprite_pos + 1))
        {
            pixel = "#";
        } else {
            pixel = ".";
        }

        if self.current_pixel.0 >= 39 {
            self.current_pixel.0 = 0;
            self.current_pixel.1 += 1;
            self.screen += pixel;
            self.screen += "\n";
        } else {
            self.current_pixel.0 += 1;
            self.screen += pixel;
        }
    }
}

fn main() {
    // let data = fs::read_to_string("test.txt").expect("Failed to read file");
    let data = fs::read_to_string("input.txt").expect("Failed to read file");

    let log_points = vec![20, 60, 100, 140, 180, 220];
    let mut cpu = Cpu::new_with_log_points(&log_points);

    for line in data.lines() {
        let mut instruction = line.split(" ");
        match instruction.next() {
            Some(a) => {
                if a == "noop" {
                    cpu.run(InstructionSet::Noop);
                } else if a == "addx" {
                    let v: i32 = instruction.next().unwrap().parse().unwrap();
                    cpu.run(InstructionSet::Addx(v));
                }
            }
            None => {}
        }
    }

    let mut signal_strength = 0;
    for i in 0..log_points.len() {
        signal_strength += log_points[i] * cpu.log[i];
    }
    println!("Part 1: {}", signal_strength);

    println!("Part 2: \n{}", cpu.crt.screen);
}
