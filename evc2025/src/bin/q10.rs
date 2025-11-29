use rustc_hash::FxHashMap;

type Pos = (usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<Vec<bool>>, Pos) {
    let mut sheep: Vec<Vec<bool>> = Vec::new();
    let mut hides: Vec<Vec<bool>> = Vec::new();
    let mut dragon = (0, 0);
    for (y, l) in input.lines().enumerate() {
        let row = l.chars().map(|c| c == 'S').collect();
        sheep.push(row);
        let hrow = l.chars().map(|c| c == '#').collect();
        hides.push(hrow);
        if let Some(x) = l.find('D') {
            dragon = (x, y);
        }
    }
    (sheep, hides, dragon)
}

fn next_pos(width: usize, height: usize, pos: Pos) -> Vec<Pos> {
    const OFFSETS: [(i32, i32); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, -1),
        (2, 1),
        (-1, -2),
        (1, -2),
    ];
    let mut out = Vec::with_capacity(8);
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    for (dx, dy) in OFFSETS {
        let new_x = x + dx;
        let new_y = y + dy;
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            out.push((new_x as usize, new_y as usize));
        }
    }
    out
}

fn part1(input: String) {
    const N_MOVES: u32 = 4;
    let (sheep, _, dragon) = parse_input(&input);

    // Find possible positions
    let mut positions = vec![vec![false; sheep[0].len()]; sheep.len()];
    positions[dragon.1][dragon.0] = true;
    let width = sheep[0].len();
    let height = sheep.len();
    for _ in 0..N_MOVES {
        for (y, row) in positions.clone().iter().enumerate() {
            for (x, _) in row.iter().enumerate().filter(|(_, f)| **f) {
                for new_pos in next_pos(width, height, (x, y)) {
                    positions[new_pos.1][new_pos.0] = true;
                }
            }
        }
    }

    // Count sheep
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if positions[y][x] && sheep[y][x] {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn part2(input: String) {
    const N_MOVES: u32 = 20;
    let (mut sheep, hides, dragon) = parse_input(&input);
    let width = sheep[0].len();
    let height = sheep.len();
    let mut count = 0u32;
    //let mut sheep_moved = 0usize;

    let mut positions = vec![vec![false; sheep[0].len()]; sheep.len()];
    positions[dragon.1][dragon.0] = true;
    for _ in 0..N_MOVES {
        // Calculate possible positions after dragon turn
        let mut next_positions = vec![vec![false; sheep[0].len()]; sheep.len()];
        for (y, row) in positions.clone().iter().enumerate() {
            for (x, _) in row.iter().enumerate().filter(|(_, f)| **f) {
                for new_pos in next_pos(width, height, (x, y)) {
                    next_positions[new_pos.1][new_pos.0] = true;
                }
            }
        }

        // Eat sheep
        for y in 0..height {
            for x in 0..width {
                if next_positions[y][x] && sheep[y][x] && !hides[y][x] {
                    count += 1;
                    sheep[y][x] = false;
                }
            }
        }

        // Move sheep
        sheep.insert(0, vec![false; width]);

        // Eat sheep again
        for y in 0..height {
            for x in 0..width {
                if next_positions[y][x] && sheep[y][x] && !hides[y][x] {
                    count += 1;
                    sheep[y][x] = false;
                }
            }
        }

        positions = next_positions;
    }
    println!("{count}");
}

fn seqs(
    hides: &[Vec<bool>],
    sheep: Vec<Option<usize>>,
    dragon: Pos,
    cache: &mut FxHashMap<(Vec<Option<usize>>, Pos), u64>,
) -> u64 {
    let width = hides[0].len();
    let height = hides.len();

    // Sheep turns
    let mut sheep_turns = Vec::with_capacity(width);
    let mut escape = false;
    for (x, opt) in sheep.iter().enumerate() {
        if let Some(y) = opt {
            let next = y + 1;
            if next == height {
                escape = true;
            } else if hides[next][x] || (x, next) != dragon {
                let mut new_sheep = sheep.clone();
                new_sheep[x] = Some(next);
                sheep_turns.push(new_sheep);
            }
        }
    }
    if sheep_turns.is_empty() && escape {
        // Branch can only end with some sheep getting away
        //cache.insert((sheep, dragon), 0);
        return 0;
    }
    if sheep_turns.is_empty() {
        // Sheep may skip a turn, replicate previous state
        sheep_turns.push(sheep.clone());
    }

    let mut ret = 0;

    // Dragon turns
    let dragon_turns = next_pos(width, height, dragon);
    for s in sheep_turns {
        for d in &dragon_turns {
            let mut cur_sheep = s.clone();
            // Eat sheep
            if cur_sheep[d.0] == Some(d.1) && !hides[d.1][d.0] {
                cur_sheep[d.0] = None;
            }
            if cur_sheep.iter().all(|s| s.is_none()) {
                // All sheep were eaten, this is an option
                ret += 1;
            } else {
                // Recurse
                if let Some(recurse) = cache.get(&(cur_sheep.clone(), *d)) {
                    ret += recurse;
                } else {
                    ret += seqs(hides, cur_sheep, *d, cache);
                }
            }
        }
    }

    // Save cache
    cache.insert((sheep, dragon), ret);
    ret
}

fn part3(input: String) {
    let (sheep, hides, dragon) = parse_input(&input);
    let width = sheep[0].len();

    // Position of the sheep in each column
    let mut sheep_cols = vec![None; width];
    for (y, row) in sheep.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, s)| **s) {
            sheep_cols[x] = Some(y);
        }
    }

    let mut cache: FxHashMap<(Vec<Option<usize>>, Pos), u64> = FxHashMap::default();
    let count = seqs(&hides, sheep_cols, dragon, &mut cache);
    println!("{count}");
}

util::evc_main!();
