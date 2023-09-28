use std::{
    cmp::{Ordering, Reverse},
    collections::VecDeque,
    fs,
};

#[derive(Debug, Clone)]
enum Operation {
    Multiply(u64),
    Square,
    Add(u64),
}

#[derive(Debug, Clone)]
enum Test {
    Divisible(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    next_monkey: [u64; 2],
    inspect_count: u64,
}

impl Monkey {
    fn inspect(&mut self, i: usize, relief: bool) -> u64 {
        self.inspect_count += 1;

        match self.operation {
            Operation::Multiply(operand) => {
                let oper = self.items[i] * operand;
                if relief {
                    oper / 3
                } else {
                    oper
                }
            }
            Operation::Square => {
                let oper = self.items[i] * self.items[i];
                if relief {
                    oper / 3
                } else {
                    oper
                }
            }
            Operation::Add(operand) => {
                let oper = self.items[i] + operand;
                if relief {
                    oper / 3
                } else {
                    oper
                }
            }
        }
    }
    fn throw(&mut self) -> (u64, u64) {
        let current = self.items.pop_front().unwrap();
        let test_result = match self.test {
            Test::Divisible(divisor) => current % divisor == 0,
        };
        if test_result {
            return (self.next_monkey[0], current);
        } else {
            return (self.next_monkey[1], current);
        }
    }
    fn inspect_and_throw(&mut self, operand_lcm: u64, relief: bool) -> Vec<(u64, u64)> {
        let mut targets: Vec<(u64, u64)> = Vec::new();
        for _i in 0..self.items.len() {
            self.inspect_count += 1;
            let changed_item: u64;
            let mut current = self.items.pop_front().unwrap();
            // It works without this, but leaving it in just to be sure.
            if !relief {
                current %= operand_lcm;
            }
            let test_result = match self.operation {
                Operation::Multiply(operand) => match self.test {
                    Test::Divisible(divisor) => {
                        if relief {
                            changed_item = current * operand / 3;
                            (current * operand / 3) % divisor
                        } else {
                            changed_item = current * operand;
                            mod_mul(current, operand, divisor)
                        }
                    }
                },
                Operation::Square => match self.test {
                    Test::Divisible(divisor) => {
                        if relief {
                            changed_item = current * current / 3;
                            (current * current / 3) % divisor
                        } else {
                            changed_item = current * current;
                            mod_mul(current, current, divisor)
                        }
                    }
                },
                Operation::Add(operand) => match self.test {
                    Test::Divisible(divisor) => {
                        if relief {
                            changed_item = (current + operand) / 3;
                            ((current + operand) / 3) % divisor
                        } else {
                            changed_item = current + operand;
                            mod_add(current, operand, divisor)
                        }
                    }
                },
            };
            if test_result == 0 {
                targets.push((self.next_monkey[0], changed_item));
            } else {
                targets.push((self.next_monkey[1], changed_item));
            }
        }
        return targets;
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
}
fn lcmm(args: &[u64]) -> u64 {
    args.to_owned()
        .into_iter()
        .reduce(|acc, a| lcm(a, acc))
        .unwrap()
}

fn mod_add(a: u64, b: u64, divisor: u64) -> u64 {
    ((a % divisor) + (b % divisor)) % divisor
}
fn mod_mul(a: u64, b: u64, divisor: u64) -> u64 {
    ((a % divisor) * (b % divisor)) % divisor
}

fn main() {
    // let data = fs::read_to_string("test.txt").expect("Failed to read file");
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let parsed: Vec<&str> = data.lines().collect();
    let monkeys: Vec<Monkey> = parsed
        .split(|&x| x.as_bytes().cmp(b"") == Ordering::Equal)
        .into_iter()
        .map(|x| {
            let items: VecDeque<u64> = x[1]
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let operation_config: Vec<&str> =
                x[2].split("old ").last().unwrap().split(" ").collect();
            let operation: Operation;
            if operation_config[0] == "*" {
                operation = match operation_config[1].parse().ok() {
                    Some(operand) => Operation::Multiply(operand),
                    None => match operation_config[1] {
                        "old" => Operation::Square,
                        _ => {
                            panic!("Failed to parse multiply operation");
                        }
                    },
                };
            } else if operation_config[0] == "+" {
                operation = Operation::Add(operation_config[1].parse().unwrap());
            } else {
                panic!("Failed to parse operation");
            }
            let test_config: Vec<&str> = x[3].split(": ").last().unwrap().split(" by ").collect();
            let test = Test::Divisible(test_config[1].parse().unwrap());
            let next_monkey: [u64; 2] = [
                x[4].split(" ").last().unwrap().parse().unwrap(),
                x[5].split(" ").last().unwrap().parse().unwrap(),
            ];
            return Monkey {
                items,
                operation,
                test,
                next_monkey,
                inspect_count: 0,
            };
        })
        .collect();

    // Get lcm of operands
    let operand_lcm = lcmm(
        monkeys
            .iter()
            .map(|x| match x.test {
                Test::Divisible(a) => a,
            })
            .collect::<Vec<u64>>()
            .iter()
            .as_slice(),
    );

    // Part 1
    let mut monkeys_part1 = monkeys.clone();

    for _round in 0..20 {
        for i in 0..monkeys_part1.len() {
            for _i in 0..monkeys_part1[i].items.len() {
                monkeys_part1[i].items[0] = monkeys_part1[i].inspect(0, true);
                let target = monkeys_part1[i].throw();
                monkeys_part1[target.0 as usize].items.push_back(target.1);
            }
        }
    }
    // for _round in 0..20 {
    //     for i in 0..monkeys_part1.len() {
    //         let targets = monkeys_part1[i].inspect_and_throw(operand_lcm, true);
    //         for target in targets {
    //             monkeys_part1[target.0 as usize].items.push_back(target.1);
    //         }
    //     }
    // }
    //
    let mut inspect_counts: Vec<u64> = monkeys_part1.iter().map(|x| x.inspect_count).collect();
    inspect_counts.sort_by_key(|x| Reverse(*x));
    println!("Part 1: {}", inspect_counts[0] * inspect_counts[1]);

    // Part 2
    let mut monkeys_part2 = monkeys.clone();

    for _round in 0..10000 {
        for i in 0..monkeys_part2.len() {
            let targets = monkeys_part2[i].inspect_and_throw(operand_lcm, false);
            for target in targets {
                monkeys_part2[target.0 as usize].items.push_back(target.1);
            }
        }
    }

    let mut inspect_counts: Vec<u64> = monkeys_part2.iter().map(|x| x.inspect_count).collect();
    inspect_counts.sort_by_key(|x| Reverse(*x));
    println!("Part 2: {}", inspect_counts[0] * inspect_counts[1]);
}
