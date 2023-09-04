use std::{borrow::BorrowMut, collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Directory<'a> {
    dirs: HashMap<&'a str, Self>,
    files: HashMap<&'a str, u32>,
}
impl<'a> Directory<'a> {
    fn new() -> Self {
        Directory {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
    fn add_file<'b: 'a>(&mut self, name: &'b str, size: u32) {
        self.files.insert(name, size);
    }
    fn add_dir<'b: 'a>(&mut self, name: &'b str) -> &mut Self {
        self.dirs.entry(name).or_insert(Self::new())
    }
    fn goto(&mut self, dir: &Vec<&str>) -> &mut Self {
        if dir.len() == 1 {
            return self
                .dirs
                .get_mut(dir[0])
                .expect("cannot move into directory");
        } else if dir.len() > 1 {
            let current_dir = self.goto(&vec![dir[0].clone()]);
            let mut dir_cloned = dir.clone();
            dir_cloned.remove(0);
            return current_dir.goto(&dir_cloned);
        } else {
            return self.borrow_mut();
        }
    }
    fn size(&self) -> u32 {
        let local_size: u32 = self.files.iter().map(|(_, x)| x).sum();
        let recursive_size: u32 = self.dirs.iter().map(|(_, x)| x.size()).sum();
        return local_size + recursive_size;
    }
    fn filter_sum_size(&self, lte: u32) -> u32 {
        let local_size: u32 = self.files.iter().map(|(_, x)| x).sum();
        let recursive_size: u32 = self.dirs.iter().map(|(_, x)| x.size()).sum();
        let recursive_filter_size: u32 =
            self.dirs.iter().map(|(_, x)| x.filter_sum_size(lte)).sum();
        if local_size + recursive_size <= lte {
            return local_size + recursive_filter_size * 2;
        } else {
            return recursive_filter_size;
        }
    }
    fn list_all_sizes<'n>(&self) -> (u32, Vec<(&str, u32)>) {
        let local_size: u32 = self.files.iter().map(|(_, x)| x).sum();
        if self.dirs.len() == 0 {
            return (local_size, vec![]);
        }
        let mut size = local_size;
        let all_sizes: Vec<(&str, u32)> = self
            .dirs
            .iter()
            .map(|(name, dir)| {
                let mut all_sizes = dir.list_all_sizes();
                let recursive_size: u32 = all_sizes.0;
                size += recursive_size;
                all_sizes.1.push((name, recursive_size));
                return all_sizes.1;
            })
            .reduce(|acc, x| {
                let mut res = acc.clone();
                res.append(&mut x.clone());
                res
            })
            .unwrap();
        return (size, all_sizes);
    }
}

enum CmdType {
    Ls,
    Cd,
}
enum LineType {
    Cmd(CmdType),
    Dir,
    File,
}

fn parse_line(line: &str) -> (LineType, Vec<&str>) {
    let v: Vec<&str> = line.split(' ').collect();
    if v[0] == "$" {
        if v[1] == "ls" {
            return (LineType::Cmd(CmdType::Ls), v[2..].to_vec());
        } else if v[1] == "cd" {
            return (LineType::Cmd(CmdType::Cd), v[2..].to_vec());
        }
    } else if v[0] == "dir" {
        return (LineType::Dir, v[1..].to_vec());
    } else if let Ok(_) = v[0].parse::<u32>() {
        return (LineType::File, v);
    }
    panic!("Failed to parse line");
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut dir_root = Directory::new();
    let mut dirs: Vec<&str> = vec![];
    let mut current_dir = dir_root.goto(&dirs);
    for line in data.lines() {
        let parsed = parse_line(line);
        match parsed.0 {
            LineType::Cmd(command) => match command {
                CmdType::Ls => {}
                CmdType::Cd => {
                    if parsed.1[0] == ".." {
                        dirs.pop();
                        current_dir = dir_root.goto(&dirs);
                    } else {
                        if parsed.1[0] == "/" {
                            dirs = vec![];
                        } else {
                            dirs.push(parsed.1[0]);
                            current_dir = dir_root.goto(&dirs);
                        }
                    }
                }
            },
            LineType::Dir => {
                current_dir.add_dir(parsed.1[0]);
            }
            LineType::File => {
                current_dir.add_file(parsed.1[1], parsed.1[0].parse::<u32>().unwrap());
            }
        }
    }
    println!("Part 1: {}", dir_root.filter_sum_size(100000));

    // Part 2
    let mut all_sizes = dir_root.list_all_sizes();
    all_sizes.1.append(&mut vec![("/", all_sizes.0)]);
    all_sizes.1.sort_by(|(_, a), (_, b)| b.cmp(a));

    let mut space_to_be_freed: u32 = 0;
    for i in 0..all_sizes.1.len() {
        if all_sizes.1[i].1 < (30000000 - (70000000 - all_sizes.0)) {
            space_to_be_freed = all_sizes.1[i - 1].1;
            break;
        }
    }
    println!("Part 2: {}", space_to_be_freed);
}
