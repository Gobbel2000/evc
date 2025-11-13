fn part1(input: String) {
    const N: i32 = 32;
    let nums: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut count = 0;
    for w in nums.windows(2) {
        let dist = (w[1] - w[0]).rem_euclid(N);
        if dist == N / 2 {
            count += 1;
        }
    }
    println!("{count}");
}

fn crosses(a: (i32, i32), b: (i32, i32)) -> bool {
    const N: i32 = 256;
    // Recenter everything around a.0 as point 0
    let bc = ((b.0 - a.0).rem_euclid(N), (b.1 - a.0).rem_euclid(N));
    let ac = (a.1 - a.0).rem_euclid(N);
    // Now ac must be in the middle of bc.0 and bc.1 for the strings to cross.
    // No end points may coincide
    (0 < bc.0 && bc.0 < ac && ac < bc.1) || (0 < bc.1 && bc.1 < ac && ac < bc.0)
}

fn part2(input: String) {
    const N: i32 = 256;
    let nums: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap() % N)
        .collect();
    let mut prev = Vec::new();
    let mut count = 0;
    for w in nums.windows(2) {
        let new = (w[0], w[1]);
        count += prev.iter().filter(|p| crosses(new, **p)).count();
        prev.push(new);
    }
    println!("{count}");
}

fn part3(input: String) {
    const N: i32 = 256;
    let nums: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap() % N)
        .collect();
    let prev: Vec<_> = nums
        .windows(2)
        // Sort all edges
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .collect();

    let mut max = 0;
    for upper in 0..N {
        for lower in 0..upper {
            let score = prev
                .iter()
                // Either crosses a string, or is exactly the same string
                .filter(|&&p| crosses((lower, upper), p) || (lower, upper) == p)
                .count();
            if score > max {
                max = score;
            }
        }
    }
    println!("{max}");
}

util::evc_main!();
