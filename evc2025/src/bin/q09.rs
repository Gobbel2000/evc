use std::collections::HashMap;

fn parse_line(input: &str) -> (usize, Vec<char>) {
    let (a, b) = input.split_once(':').unwrap();
    (a.parse().unwrap(), b.chars().collect())
}

fn part1(input: String) {
    let scales: Vec<_> = input.lines().map(|l| parse_line(l).1).collect();
    for (idx, s) in scales.iter().enumerate() {
        let others: Vec<_> = scales
            .iter()
            .enumerate()
            .filter(|(idx2, _)| *idx2 != idx)
            .map(|(_, e)| e)
            .collect();
        let mut sim = vec![0; others.len()];
        let mut child = true;
        for i in 0..s.len() {
            let cur = s[i];
            if cur != others[0][i] && cur != others[1][i] {
                child = false;
                break;
            }
            if cur == others[0][i] {
                sim[0] += 1;
            }
            if cur == others[1][i] {
                sim[1] += 1;
            }
        }
        if child {
            println!("{}", sim[0] * sim[1]);
        }
    }
}

fn sim(child: &[char], p1: &[char], p2: &[char]) -> Option<u32> {
    let mut sim = [0, 0];
    let mut is_child = true;
    for i in 0..child.len() {
        let cur = child[i];
        if cur != p1[i] && cur != p2[i] {
            is_child = false;
            break;
        }
        if cur == p1[i] {
            sim[0] += 1;
        }
        if cur == p2[i] {
            sim[1] += 1;
        }
    }
    is_child.then(|| sim[0] * sim[1])
}

fn part2(input: String) {
    let scales: Vec<_> = input.lines().map(|l| parse_line(l).1).collect();
    let mut sum = 0;
    for (ci, child) in scales.iter().enumerate() {
        for (p1i, p1) in scales.iter().enumerate().filter(|(idx, _)| *idx != ci) {
            for (_p2i, p2) in scales
                .iter()
                .take(p1i)
                .enumerate()
                .filter(|(idx, _)| *idx != ci && *idx != p1i)
            {
                if let Some(s) = sim(child, p1, p2) {
                    sum += s;
                }
            }
        }
    }
    println!("{sum}");
}

fn is_child(child: &[char], p1: &[char], p2: &[char]) -> bool {
    child
        .iter()
        .zip(p1)
        .zip(p2)
        .all(|((c, a), b)| c == a || c == b)
}

fn join_family(c: usize, p1: usize, p2: usize, family: &mut [usize]) {
    let new_id = family[c];
    let old_p1 = family[p1];
    let old_p2 = family[p2];
    for e in family.iter_mut() {
        if *e == old_p1 || *e == old_p2 {
            *e = new_id;
        }
    }
}

fn part3(input: String) {
    let scales: Vec<_> = input.lines().map(parse_line).collect();
    let mut family: Vec<usize> = (0..scales.len() + 1).collect();
    for (ci, child) in scales.iter().enumerate() {
        for (p1i, p1) in scales.iter().enumerate().filter(|(idx, _)| *idx != ci) {
            for (_p2i, p2) in scales
                .iter()
                .take(p1i)
                .enumerate()
                .filter(|(idx, _)| *idx != ci)
            {
                if is_child(&child.1, &p1.1, &p2.1) {
                    join_family(child.0, p1.0, p2.0, &mut family);
                }
            }
        }
    }

    let mut size: HashMap<usize, u32> = HashMap::new();
    for e in &family {
        *size.entry(*e).or_default() += 1;
    }
    let biggest = size.iter().max_by_key(|(_, s)| *s).unwrap().0;
    let mut sum = 0;
    for (id, f) in family.iter().enumerate() {
        if f == biggest {
            sum += id;
        }
    }
    println!("{sum}");
}

util::evc_main!();
