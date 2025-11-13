fn part1(input: String) {
    let mut sum = 0;
    for (idx, c) in input.chars().enumerate() {
        if c == 'a' {
            for c2 in input.chars().take(idx) {
                if c2 == 'A' {
                    sum += 1;
                }
            }
        }
    }
    println!("{sum}");
}

fn part2(input: String) {
    let mut sum = 0;
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_lowercase() {
            for c2 in input.chars().take(idx) {
                if c2 == c.to_ascii_uppercase() {
                    sum += 1;
                }
            }
        }
    }
    println!("{sum}");
}

fn part3(input: String) {
    const WIDTH: i64 = 1000;
    const REP: u32 = 1000;
    const SEARCH: i64 = 2 * WIDTH + 1;
    let n = input.len() as i64;
    assert!(n > SEARCH);

    let mut sum = 0;
    // Middle repetitions, range always wraps around
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_lowercase() {
            for c2 in input.chars().cycle()
                    .skip((idx as i64 - WIDTH).rem_euclid(n) as usize)
                    .take(SEARCH as usize) {
                if c2 == c.to_ascii_uppercase() {
                    sum += 1;
                }
            }
        }
    }
    sum *= REP - 2;

    // First repetition, clamp at start
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_lowercase() {
            for c2 in input.chars().cycle()
                    .skip((idx as i64 - WIDTH).max(0) as usize)
                    .take(SEARCH.min(idx as i64 + WIDTH + 1) as usize) {
                if c2 == c.to_ascii_uppercase() {
                    sum += 1;
                }
            }
        }
    }

    // Last repetition, clamp at end
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_lowercase() {
            for c2 in input.chars().cycle()
                    .skip((idx as i64 - WIDTH).rem_euclid(n) as usize)
                    .take(SEARCH.min(n - idx as i64 + WIDTH) as usize) {
                if c2 == c.to_ascii_uppercase() {
                    sum += 1;
                }
            }
        }
    }
    println!("{sum}");
}

util::evc_main!();
