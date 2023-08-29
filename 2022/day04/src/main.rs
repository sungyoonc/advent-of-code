use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read line.");
    // ------- Part 1 -------
    let pairs_list: Vec<Vec<Vec<i32>>> = data // split string to vectors of int
        .lines()
        .map(|x| {
            x.split(',')
                .map(|y| {
                    y.split('-')
                        .map(|z| z.parse::<i32>().expect("Unable to parse str to i32"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();
    let mut counter = 0;
    for pairs in pairs_list.iter() {
        if !(pairs[0][0] < pairs[1][0] && pairs[0][1] < pairs[1][1]) {
            if !(pairs[0][0] > pairs[1][0] && pairs[0][1] > pairs[1][1]) {
                counter += 1;
            }
        }
    }
    println!("Part 1: {}", counter);

    // ------- Part 2 -------
    let mut counter = 0;
    for pairs in pairs_list {
        if !(pairs[0][1] < pairs[1][0] || pairs[0][0] > pairs[1][1]) {
            counter += 1;
        }
    }
    println!("Part 2: {}", counter);
}
