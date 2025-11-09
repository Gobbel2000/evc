use std::cmp::Ordering;

#[derive(Debug)]
struct Segment {
    left: Option<u64>,
    spine: u64,
    right: Option<u64>,
}

impl Segment {
    fn new(spine: u64) -> Self {
        Segment { left: None, spine, right: None }
    }

    fn sum(&self) -> u64 {
        let mut sum = 0;
        if let Some(l) = self.left {
            sum = l * 10;
        }
        sum += self.spine;
        if let Some(r) = self.right {
            sum *= 10;
            sum += r;
        }
        sum
    }
}

fn cmp_fishbones(a: &[Segment], b: &[Segment]) -> Ordering {
    let qual_a = sword_quality(a);
    let qual_b = sword_quality(b);
    if qual_a != qual_b {
        return qual_a.cmp(&qual_b)
    }
    let sums_a = a.iter().map(Segment::sum).collect::<Vec<u64>>();
    let sums_b = b.iter().map(Segment::sum).collect::<Vec<u64>>();
    sums_a.cmp(&sums_b)
}

fn parse_sword(input: &str) -> (u64, Vec<u64>) {
    let (id, numss) = input.split_once(':').unwrap();
    (id.parse().unwrap(),
        numss.split(',').map(|w| w.parse().unwrap()).collect())
}

fn fishbone(sword: &[u64]) -> Vec<Segment> {
    let mut fishbone: Vec<Segment> = Vec::new();
    'outer: for &n in sword {
        for bone in &mut fishbone {
            if bone.left.is_none() && n < bone.spine {
                bone.left = Some(n);
                continue 'outer
            } else if bone.right.is_none() && n > bone.spine {
                bone.right = Some(n);
                continue 'outer
            }
        }
        fishbone.push(Segment::new(n));
    }
    fishbone
}

fn sword_quality(bones: &[Segment]) -> u64 {
    let mut quality = 0;
    for bone in bones {
        quality *= 10;
        quality += bone.spine;
    }
    quality
}
fn quality(sword: &[u64]) -> u64 {
    let bones = fishbone(sword);
    sword_quality(&bones)
}

fn part1(input: String) {
    println!("{}", quality(&parse_sword(&input).1));
}

fn part2(input: String) {
    let swords: Vec<u64> = input.lines().map(|l| quality(&parse_sword(l).1)).collect();
    let min = swords.iter().min().unwrap();
    let max = swords.iter().max().unwrap();
    println!("{}", max - min);
}

fn part3(input: String) {
    let mut swords: Vec<(u64, Vec<Segment>)> = input.lines().map(|l| {
        let (id, nums) = parse_sword(l);
        (id, fishbone(&nums))
    }).collect();
    swords.sort_by(|a, b| cmp_fishbones(&a.1, &b.1));
    swords.reverse();
    let mut checksum = 0;
    for (i, (id, _)) in swords.iter().enumerate() {
        checksum += (i as u64 + 1) * id;
    }
    println!("{checksum}");
}

util::evc_main!();
