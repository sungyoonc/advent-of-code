use std::{collections::HashSet, fs};

fn test_unique(characters: Vec<char>) -> bool {
    let mut unique = HashSet::new();
    characters.into_iter().all(|x| unique.insert(x.clone()))
}

fn get_marker(signal: &String, length: u32) -> u32 {
    let mut marker: usize = 0;
    let length = length as usize;
    for i in 0..=(signal.len() - length) {
        let characters: Vec<char> = signal[i..(i + length)].chars().collect();
        if test_unique(characters) {
            marker = i + length;
            break;
        }
    }
    return marker as u32;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file.");

    // ------- Part 1 -------
    let marker = get_marker(&data, 4);
    println!("Part 1: {}", marker);
    
    // ------- Part 2 -------
    let marker = get_marker(&data, 14);
    println!("Part 2: {}", marker);
}
