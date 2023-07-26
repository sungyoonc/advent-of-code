use std::{cmp::Ordering, fs};

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file.");

    let mut parsed: Vec<i32> = data
        .lines()
        .collect::<Vec<&str>>()
        .split(|x| x.bytes().cmp(*b"") == Ordering::Equal) // Split by blank line
        .into_iter()
        // Parse &str to i32 and sum the number
        .map(|x| x.into_iter().map(|&x| x.parse::<i32>().unwrap()).sum())
        .collect();


    // Elf carrying the most Calories
    println!(
        "{:?}",
        parsed.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
    );
    
    // Sum of calories that top 3 elves are carrying
    parsed.sort();
    let sorted: Vec<&i32> = parsed.iter().rev().collect();
    println!("{} {} {}", sorted[0], sorted[1], sorted[2]);
    println!("{:?}", sorted[0] + sorted[1] + sorted[2]);
}
