fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let names_line = lines.next().unwrap();
    lines.next().unwrap();
    let instructions_line = lines.next().unwrap();

    let names: Vec<&str> = names_line.split(',').collect();
    let instructions: Vec<&str> = instructions_line.split(',').collect();
    (names, instructions)
}

fn part1(input: String) {
    let (names, instructions) = parse_input(&input);

    let mut cur: i32 = 0;
    let n = names.len() as i32;
    for ins in instructions {
        let dir = &ins[0..1];
        let mut amount: i32 = ins[1..].parse().unwrap();
        if dir == "L" {
            amount = -amount; 
        }
        cur = (cur + amount).clamp(0, n - 1);
    }
    println!("{}", names[cur as usize]);
}

fn part2(input: String) {
    let (names, instructions) = parse_input(&input);

    let mut cur: i32 = 0;
    let n = names.len() as i32;
    for ins in instructions {
        let dir = &ins[0..1];
        let mut amount: i32 = ins[1..].parse().unwrap();
        if dir == "L" {
            amount = -amount; 
        }
        cur = (cur + amount).rem_euclid(n);
    }
    println!("{}", names[cur as usize]);
}

fn part3(input: String) {
    let (mut names, instructions) = parse_input(&input);

    let n = names.len();
    for ins in instructions {
        let dir = &ins[0..1];
        let mut idx = ins[1..].parse::<usize>().unwrap() % n;
        if dir == "L" {
            idx = (n - idx) % n;
        }
        names.swap(0, idx);
    }
    println!("{}", names[0]);
}

util::evc_main!();
