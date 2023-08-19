use std::fs;

#[derive(Debug)]
struct Instructions {
    amount: usize,
    source: usize,
    dest: usize,
}

#[derive(Debug)]
struct Crane {
    status: Vec<Vec<char>>,
    instructions: Vec<Instructions>,
}

impl Crane {
    fn from(data: &String) -> Self {
        let stack_length = (data.lines().next().unwrap().len() + 1) / 4;
        let mut status = vec![Vec::new(); stack_length];
        let mut instructions = Vec::new();
        let mut switch: bool = false;
        for line in data.lines() {
            if line.len() == 0 {
                switch = true;
            }

            // Parse location of crates
            if switch == false {
                for i in 0..((line.len() + 1) / 4) {
                    if line.chars().nth(i * 4).unwrap() == '[' {
                        let current_crate: char = line.chars().nth(i * 4 + 1).unwrap();
                        if current_crate != ' ' {
                            status[i].insert(0, current_crate);
                        }
                    }
                }
            } else if line.len() != 0 {
                let split_line: Vec<&str> = line.split(' ').collect();
                let a: Instructions = Instructions {
                    amount: split_line[1].parse().unwrap(),
                    source: split_line[3].parse().unwrap(),
                    dest: split_line[5].parse().unwrap(),
                };
                instructions.push(a);
            }
        }
        Crane {
            status,
            instructions,
        }
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read line.");

    // ------- Part 1 -------
    let mut crane = Crane::from(&data);
    for a in &mut crane.instructions {
        for _i in 0..a.amount {
            let current_crate = crane.status[a.source - 1].pop().unwrap();
            crane.status[a.dest - 1].push(current_crate);
        }
    }

    let top_crates = crane
        .status
        .iter()
        .map(|x| x[x.len() - 1].to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("Part 1: {}", top_crates);

    // ------- Part 2 -------
    let mut crane = Crane::from(&data);
    for a in &mut crane.instructions {
        let len = crane.status[a.source - 1].len();
        let mut current_crates: Vec<_> = crane.status[a.source - 1].drain((len - a.amount)..).collect();
        crane.status[a.dest - 1].append(&mut current_crates);
    }

    let top_crates = crane
        .status
        .iter()
        .map(|x| x[x.len() - 1].to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("Part 2: {}", top_crates);
}
