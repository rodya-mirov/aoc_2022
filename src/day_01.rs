fn input() -> String {
    std::fs::read_to_string("input/input_01.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> u32 {
    let mut max_elf_amt = u32::MIN;
    let mut current_elf_amt = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current_elf_amt > max_elf_amt {
                max_elf_amt = current_elf_amt;
            }
            current_elf_amt = 0;
            continue;
        }

        let amt = line.parse::<u32>().expect("Line should be empty or an int");
        current_elf_amt += amt;
    }

    if current_elf_amt > max_elf_amt {
        max_elf_amt = current_elf_amt;
    }

    max_elf_amt
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> u32 {
    let mut max_elves = Vec::with_capacity(4);

    let mut current_elf_amt = 0;

    for line in input.lines() {
        if line.is_empty() {
            max_elves.push(current_elf_amt);
            if max_elves.len() > 3 {
                max_elves.sort_by(|a, b| b.cmp(a));
                max_elves.pop();
            }

            current_elf_amt = 0;
            continue;
        }

        let amt = line.parse::<u32>().expect("Line should be empty or an int");
        current_elf_amt += amt;
    }

    max_elves.push(current_elf_amt);
    if max_elves.len() > 3 {
        max_elves.sort_by(|a, b| b.cmp(a));
        max_elves.pop();
    }

    max_elves.into_iter().sum()
}
