fn part1(input: String) {
    let mut crates: Vec<u32> = input.split(",").map(|w| w.parse().unwrap()).collect();
    crates.sort_unstable();
    crates.dedup();
    println!("{}", crates.iter().sum::<u32>());
}

fn part2(input: String) {
    let mut crates: Vec<u32> = input.split(",").map(|w| w.parse().unwrap()).collect();
    crates.sort_unstable();
    crates.dedup();
    println!("{}", crates.iter().take(20).sum::<u32>());
}

fn part3(input: String) {
    let mut crates: Vec<u32> = input.split(",").map(|w| w.parse().unwrap()).collect();
    crates.sort_unstable();
    let mut max = 0;
    let mut cur = 0;
    let mut prev = u32::MAX;
    for e in crates {
        cur += 1;
        if e != prev {
            if cur > max {
                max = cur;
            }
            cur = 0;
        }
        prev = e;
    }
    println!("{max}");
}

util::evc_main!();
