use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<&str>, HashMap<char, Vec<char>>) {
    let mut lines = input.lines();
    let l1 = lines.next().unwrap();
    let names = l1.split(',').collect();

    let mut rules = HashMap::new();
    let _ = lines.next().unwrap();
    for l in lines {
        let (a, b) = l.split_once(" > ").unwrap();
        let possibilities = b.split(',').map(|s| s.chars().next().unwrap()).collect();
        rules.insert(a.chars().next().unwrap(), possibilities);
    }

    (names, rules)
}

fn part1(input: String) {
    let (names, rules) = parse_input(&input);
    'outer: for n in names {
        let mut it = n.chars().peekable();
        while let Some(cur) = it.next()
            && let Some(next) = it.peek()
        {
            if let Some(allowed) = rules.get(&cur)
                && !allowed.contains(next)
            {
                continue 'outer;
            }
        }
        println!("{n}");
    }
}

fn part2(input: String) {
    let (names, rules) = parse_input(&input);
    let mut sum = 0;
    'outer: for (idx, n) in names.iter().enumerate() {
        let mut it = n.chars().peekable();
        while let Some(cur) = it.next()
            && let Some(next) = it.peek()
        {
            if let Some(allowed) = rules.get(&cur)
                && !allowed.contains(next)
            {
                continue 'outer;
            }
        }
        sum += idx + 1;
    }
    println!("{sum}");
}

const MIN: usize = 7;
const MAX: usize = 11;

fn rec_ext(
    last: char,
    missing: usize,
    cache: &mut HashMap<(char, usize), u64>,
    rules: &HashMap<char, Vec<char>>,
) -> u64 {
    if missing == 0 {
        return 0;
    }
    if let Some(res) = cache.get(&(last, missing)) {
        return *res;
    }
    let Some(next) = rules.get(&last) else {
        return 0;
    };
    let num = next.len();
    let mut res = 0;
    if missing <= (MAX - MIN + 1) {
        res = num as u64;
    }
    for n in next {
        res += rec_ext(*n, missing - 1, cache, rules);
    }
    cache.insert((last, missing), res);
    res
}

fn extensions(
    name: &str,
    cache: &mut HashMap<(char, usize), u64>,
    rules: &HashMap<char, Vec<char>>,
) -> u64 {
    debug_assert!(name.len() <= MAX);
    rec_ext(name.chars().last().unwrap(), MAX - name.len(), cache, rules)
}

fn part3(input: String) {
    let (mut names, rules) = parse_input(&input);
    // Sort to start with shorter prefixes first.
    names.sort();
    let mut sum = 0;

    // Global dynamic programming cache
    let mut cache = HashMap::new();
    'outer: for (idx, n) in names.iter().enumerate() {
        // We have to sort out prefixes that are already covered before
        for np in names.iter().take(idx) {
            if n.starts_with(np) {
                continue 'outer;
            }
        }
        let mut it = n.chars().peekable();
        while let Some(cur) = it.next()
            && let Some(next) = it.peek()
        {
            if let Some(allowed) = rules.get(&cur)
                && !allowed.contains(next)
            {
                continue 'outer;
            }
        }
        sum += extensions(n, &mut cache, &rules);
    }
    println!("{sum}");
}

util::evc_main!();
