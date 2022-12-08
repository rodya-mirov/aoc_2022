use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/input_07.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let root_dir = parse_tree_from_input(input);

    const CAP_SIZE: usize = 100000;

    fn traverse_tree(dir: &Directory) -> usize {
        let mut total = 0;

        if dir.size < CAP_SIZE {
            total += dir.size;
        }

        for child in dir.contents.iter() {
            if let DiskObj::Dir(d) = child {
                total += traverse_tree(d);
            }
        }

        total
    }

    traverse_tree(&root_dir)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let root_dir = parse_tree_from_input(input);

    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = root_dir.size;
    let free_space = total_space - used_space;
    let minimum_delete = needed_space - free_space;

    fn find_best_delete(d: &Directory, min_delete: usize) -> Option<usize> {
        if d.size < min_delete {
            None
        } else {
            // delete self if none of the children are big enough
            let mut best_delete = d.size;
            for child in d.contents.iter() {
                if let DiskObj::Dir(d) = child {
                    let best_child_delete = find_best_delete(d, min_delete);
                    best_delete = match best_child_delete {
                        None => best_delete,
                        Some(running) => running.min(best_delete),
                    };
                }
            }
            Some(best_delete)
        }
    }

    find_best_delete(&root_dir, minimum_delete).expect("Root dir should be big enough to delete")
}

fn parse_tree_from_input(input_str: &str) -> Directory {
    let mut lines = input_str.trim().lines().peekable();

    let first_line = lines.next().expect("Lines should be nonempty");

    assert_eq!(first_line, "$ cd /");

    let mut working_stack: Vec<Directory> = Vec::new();
    let mut working_dir = Directory::new();

    let into_dir_pattern = Regex::new(r"^\$ cd ([A-Za-z]+)$").expect("Regex should compile");
    let dir_statement_pattern = Regex::new(r"^dir ([A-Za-z]+)$").expect("Regex should compile");
    let file_size_pattern = Regex::new(r"^([0-9]+) ([A-Za-z\.]+)$").expect("Regex should compile");

    // are we SURE we don't want a parsing library, hmmm
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            assert!(!working_stack.is_empty(), "Should not popd at root");
            let mut parent_dir = working_stack.pop().unwrap();
            parent_dir.add_child(DiskObj::Dir(working_dir));
            working_dir = parent_dir;
        } else if into_dir_pattern.is_match(line) {
            working_stack.push(working_dir);
            // interestingly, we never need the directory name, so we don't need to parse it
            // it does make printing the tree weird (which we also don't need to do, but still)
            working_dir = Directory::new();
        } else if line == "$ ls" {
            while let Some(next_line) = lines.peek() {
                if dir_statement_pattern.is_match(next_line) {
                    // do nothing with the line; we'll write it down once we CD into it
                    lines.next();
                } else if file_size_pattern.is_match(next_line) {
                    let capture = file_size_pattern.captures_iter(next_line).next().unwrap();
                    let size: usize = capture[1].parse().unwrap();
                    // note: intentionally skipping name here since we don't need it
                    let file = File { size };
                    working_dir.add_child(DiskObj::File(file));
                    lines.next();
                } else if next_line.starts_with('$') {
                    // done with the ls results; don't consume this line, we'll want it later
                    break;
                } else {
                    unimplemented!("Unexpected output while parsing ls: {}", next_line);
                }
            }
        } else {
            unimplemented!("Bad input: {}", line);
        }
    }

    while let Some(mut parent_dir) = working_stack.pop() {
        parent_dir.add_child(DiskObj::Dir(working_dir));
        working_dir = parent_dir;
    }

    working_dir
}

enum DiskObj {
    Dir(Directory),
    File(File),
}

impl DiskObj {
    fn size(&self) -> usize {
        match self {
            DiskObj::Dir(d) => d.size,
            DiskObj::File(f) => f.size,
        }
    }
}

struct File {
    size: usize,
}

struct Directory {
    contents: Vec<DiskObj>,
    size: usize,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            contents: Vec::new(),
            size: 0,
        }
    }

    fn add_child(&mut self, child: DiskObj) {
        self.size += child.size();
        self.contents.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_INPUT_STR), 95437);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_INPUT_STR), 24933642);
    }
}
