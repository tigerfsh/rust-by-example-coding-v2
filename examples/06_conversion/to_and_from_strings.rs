use std::fmt;

// Converting to String
// 实现Display, 自动实现ToString

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// Parsing a String

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct CircleV2 {
    radius: i32,
}

impl FromStr for CircleV2 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(CircleV2 { radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {sum:?}");

    let radius = "    3 ";
    let circle: CircleV2 = radius.parse().unwrap();
    println!("{circle:?}")
}
