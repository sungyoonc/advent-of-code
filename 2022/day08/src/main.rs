use std::fs;

#[derive(Debug)]
struct Visibility {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

#[derive(Debug)]
struct Score {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: Visibility,
    score: Score,
}
impl Tree {
    fn is_visible(&self) -> bool {
        let visibility =
            self.visible.left || self.visible.right || self.visible.top || self.visible.bottom;
        return visibility;
    }
    fn scenic_score(&self) -> u32 {
        let total_score = self.score.left * self.score.right * self.score.top * self.score.bottom;
        return total_score;
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    column_count: u32,
    row_count: u32,
}
impl Forest {
    fn from(data: String) -> Self {
        let column_count = data.lines().next().unwrap().len() as u32; // Number of column (Horizontal length)
        let row_count = data.lines().count() as u32; // Number of rows (Vertical length)
        let data_parsed: Vec<Vec<Tree>> = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| Tree {
                        height: x.to_digit(10).expect("Failed to parse height"),
                        visible: Visibility {
                            left: false,
                            right: false,
                            top: false,
                            bottom: false,
                        },
                        score: Score {
                            left: 0,
                            right: 0,
                            top: 0,
                            bottom: 0,
                        },
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect();
        let trees = data_parsed;
        return Self {
            trees,
            column_count,
            row_count,
        };
    }
    fn update_visibilities(&mut self) {
        for y in 0..(self.row_count as usize) {
            // Update left visibility
            let mut height_tallest: u32 = 0;
            for x in 0..(self.column_count as usize) {
                let mut current_tree = &mut self.trees[y][x];
                if current_tree.height > height_tallest || x == 0 {
                    height_tallest = current_tree.height;
                    current_tree.visible.left = true;
                } else {
                    current_tree.visible.left = false;
                }
            }
            // Update right visibility
            let mut height_tallest: u32 = 0;
            for x in (0..(self.column_count as usize)).rev() {
                let mut current_tree = &mut self.trees[y][x];
                if current_tree.height > height_tallest || x == (self.column_count - 1) as usize {
                    height_tallest = current_tree.height;
                    current_tree.visible.right = true;
                } else {
                    current_tree.visible.right = false;
                }
            }
        }
        for x in 0..(self.column_count as usize) {
            // Update top visibility
            let mut height_tallest: u32 = 0;
            for y in 0..(self.row_count as usize) {
                let mut current_tree = &mut self.trees[y][x];
                if current_tree.height > height_tallest || y == 0 {
                    height_tallest = current_tree.height;
                    current_tree.visible.top = true;
                } else {
                    current_tree.visible.top = false;
                }
            }
            // Update bottom visibility
            let mut height_tallest: u32 = 0;
            for y in (0..(self.row_count as usize)).rev() {
                let mut current_tree = &mut self.trees[y][x];
                if current_tree.height > height_tallest || y == (self.row_count - 1) as usize {
                    height_tallest = current_tree.height;
                    current_tree.visible.bottom = true;
                } else {
                    current_tree.visible.bottom = false;
                }
            }
        }
    }
    fn update_score(&mut self) {
        for y in 0..(self.row_count as usize) {
            for x in 0..(self.column_count as usize) {
                self.trees[y][x].score = Score {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                };
                let current_height = self.trees[y][x].height;

                // Update left score
                let mut score_count: u32 = 0;
                for i in (0..x).rev() {
                    score_count += 1;
                    if self.trees[y][i].height >= current_height {
                        break;
                    }
                }
                self.trees[y][x].score.left = score_count;

                // Update right score
                let mut score_count: u32 = 0;
                for i in (x + 1)..(self.column_count as usize) {
                    score_count += 1;
                    if self.trees[y][i].height >= current_height {
                        break;
                    }
                }
                self.trees[y][x].score.right = score_count;
            }
        }
        for x in 0..(self.column_count as usize) {
            for y in 0..(self.row_count as usize) {
                // Doesn't reset the score because it's already done above
                let current_height = self.trees[y][x].height;

                // Updaste top score
                let mut score_count: u32 = 0;
                for i in (0..y).rev() {
                    score_count += 1;
                    if self.trees[i][x].height >= current_height {
                        break;
                    }
                }
                self.trees[y][x].score.top = score_count;

                // Updaste bottom score
                let mut score_count: u32 = 0;
                for i in (y + 1)..(self.row_count as usize) {
                    score_count += 1;
                    if self.trees[i][x].height >= current_height {
                        break;
                    }
                }
                self.trees[y][x].score.bottom = score_count;
            }
        }
    }
    fn visible_count(&self) -> u32 {
        let count: u32 = self
            .trees
            .iter()
            .map(|y| {
                y.iter()
                    .map(|x| match x.is_visible() {
                        true => 1,
                        false => 0,
                    })
                    .sum::<u32>()
            })
            .sum();
        return count;
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut forest = Forest::from(data);

    // Part 1
    forest.update_visibilities();
    println!("Part 1: {}", forest.visible_count());

    // Part 2
    forest.update_score();
    let top_score = forest
        .trees
        .iter()
        .map(|row| row.iter().map(|tree| tree.scenic_score()).max().unwrap())
        .max()
        .unwrap();
    println!("Part 2: {}", top_score);
}
