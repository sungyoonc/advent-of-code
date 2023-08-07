use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read line.");
    let mut list: Vec<String> = Vec::new();

    // ------- Part 1 -------
    for a in data.lines() {
        let items = &a[..(a.len() / 2)];
        let right_items = &a[(a.len() / 2)..];
        let res: String = items
            .chars()
            .filter_map(|x| match right_items.contains(x) {
                true => Some(x),
                false => None,
            })
            .collect();
        if res.len() > 0 {
            list.push((&res[..1]).to_string());
        }
    }
    let priority_sum: u32 = list
        .iter()
        .map(|x| {
            let priority = x.as_bytes()[0] - b'A';
            if priority <= 25 {
                priority as u32 + 26 + 1
            } else {
                priority as u32 - 32 + 1
            }
        })
        .sum();
    println!("Part 1. Sum is: {}", priority_sum);

    // ------- Part 2 -------
    let original_vec: Vec<String> = data.lines().map(|x| x.to_string()).collect();
    let split_vec: Vec<Vec<_>> = original_vec.chunks(3).map(|chunk| chunk.to_vec()).collect();
    let badges: Vec<char> = split_vec
        .iter()
        .map(|x| {
            for a in x[0].chars() {
                if x[1].contains(a) && x[2].contains(a) {
                    return a;
                }
            }
            panic!("Cannot find common char(a.k.a badge)");
        })
        .collect();
    let priority_sum: u32 = badges
        .into_iter()
        .map(|x| {
            let priority = x as u8 - b'A';
            if priority <= 25 {
                priority as u32 + 26 + 1
            } else {
                priority as u32 - 32 + 1
            }
        })
        .sum();
    println!("Part 2. Sum is: {}", priority_sum);
}
