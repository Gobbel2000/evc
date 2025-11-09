use std::fmt;
use std::str::FromStr;
use std::ops::{Add, Mul, Div};

#[derive(Clone, Copy)]
struct Complex {
    real: i64,
    imag: i64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.real, self.imag)
    }
}

impl FromStr for Complex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err("Does not start with '['".to_owned())
        }
        if !s.ends_with(']') {
            return Err("Does not end with '['".to_owned())
        }
        let s2 = &s[1..s.len()-1];
        let (a, b) = s2.split_once(',').ok_or("Missing ','".to_owned())?;
        let real = a.parse::<i64>().map_err(|e| e.to_string())?; 
        let imag = b.parse::<i64>().map_err(|e| e.to_string())?; 
        Ok(Complex { real, imag })
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Complex {
            real: self.real * other.real - (self.imag * other.imag),
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Complex {
            real: self.real / other.real,
            imag: self.imag / other.imag,
        }
    }
}

fn parse_input(input: &str) -> Complex {
    input[2..].parse().unwrap()
}

fn part1(input: String) {
    let a = parse_input(&input);    
    let mut acc = Complex { real: 0, imag: 0 };
    let div = Complex { real: 10, imag: 10};
    for _ in 0..3 {
        acc = acc * acc;
        acc = acc / div;
        acc = acc + a;
    }
    println!("{acc}");
}

fn part2(input: String) {
    const MAX: i64 = 1000000;
    let a = parse_input(&input);
    let mut count: u32 = 0;
    let div = Complex { real: 100000, imag: 100000 };
    for x in (a.real..=a.real+1000).step_by(10) {
        for y in (a.imag..=a.imag+1000).step_by(10) {
            let p = Complex { real: x, imag: y };
            let mut acc = Complex { real: 0, imag: 0 };
            let mut paint = true;
            for _ in 0..100 {
                acc = acc * acc;
                acc = acc / div;
                acc = acc + p;
                if acc.real < -MAX || acc.real > MAX || acc.imag < -MAX || acc.imag > MAX {
                    paint = false;
                    break;
                }
            }
            if paint {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn print_grid(grid: &[Vec<bool>]) {
    for y in (0..grid.len()).step_by(4) {
        for x in (0..grid[0].len()).step_by(2) {
            let char = match (grid[x][y], grid[x+1][y], grid[x][y+2], grid[x+1][y+2]) {
                (false, false, false, false) => ' ',
                (false, false, false, true) => '▗',
                (false, false, true, false) => '▖',
                (false, false, true, true) => '▄',
                (false, true, false, false) => '▝',
                (false, true, false, true) => '▐',
                (false, true, true, false) => '▞',
                (false, true, true, true) => '▟',
                (true, false, false, false) => '▘',
                (true, false, false, true) => '▚',
                (true, false, true, false) => '▌',
                (true, false, true, true) => '▙',
                (true, true, false, false) => '▀',
                (true, true, false, true) => '▜',
                (true, true, true, false) => '▛',
                (true, true, true, true) => '█',
            };
            print!("{char}");
        }
        println!("||");
    }
}

fn part3(input: String) {
    const MAX: i64 = 1000000;
    let a = parse_input(&input);
    //let a = Complex { real: 35300, imag: -64910 };
    let mut count: u32 = 0;
    let div = Complex { real: 100000, imag: 100000 };
    let mut grid = vec![vec![false; 1004]; 1004];
    for x in 0..=1000 {
        for y in 0..=1000 {
            let p = Complex { real: a.real + x, imag: a.imag + y };
            let mut acc = Complex { real: 0, imag: 0 };
            let mut paint = true;
            for _ in 0..100 {
                acc = acc * acc;
                acc = acc / div;
                acc = acc + p;
                if acc.real < -MAX || acc.real > MAX || acc.imag < -MAX || acc.imag > MAX {
                    paint = false;
                    break;
                }
            }
            if paint {
                count += 1;
                grid[y as usize][x as usize] = true;
            }
        }
    }
    println!("{count}");
    print_grid(&grid);
}

util::evc_main!();
