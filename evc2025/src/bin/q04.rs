use num_rational::Rational64;

fn part1(input: String) {
    let gears: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    //let mut acc = Rational32::new(2025, 1);
    let result = 2025 * gears[0] / gears[gears.len() - 1];
    println!("{result}");
}

fn part2(input: String) {
    const LAST: u64 = 10000000000000;
    let gears: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let first = gears.first().unwrap();
    let last = gears.last().unwrap();
    let result = (LAST * last).div_ceil(*first);
    println!("{result}");
}

fn part3(input: String) {
    let mut lines = input.lines();
    let first: i64 = lines.next().unwrap().parse().unwrap();
    let mut doubles: Vec<(i64, i64)> = Vec::new();
    for l in lines.clone().take_while(|l| l.contains('|')) {
        let (a, b) = l.split_once('|').unwrap();
        doubles.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    let last: i64 = lines.last().unwrap().parse().unwrap();

    let mut turns = Rational64::from(100);
    turns *= Rational64::new(first, doubles.first().unwrap().0);
    for gears in doubles.windows(2) {
        turns *= Rational64::new(gears[0].1, gears[1].0);
    }
    turns *= Rational64::new(doubles.last().unwrap().1, last);
    println!("{}", turns.to_integer());
}

util::evc_main!();
