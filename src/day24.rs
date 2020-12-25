use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::many0, IResult};

#[derive(Debug, Hash, PartialEq, Eq)]
struct HexCoord {
    x: i64,
    y: i64,
}

#[allow(clippy::unnecessary_wraps)]
fn map_dir_str(dir: &str) -> Result<HexCoord, ()> {
    Ok(match dir {
        "e" => HexCoord { x: 1, y: 0 },
        "ne" => HexCoord { x: 0, y: 1 },
        "nw" => HexCoord { x: -1, y: 1 },
        "w" => HexCoord { x: -1, y: 0 },
        "sw" => HexCoord { x: 0, y: -1 },
        "se" => HexCoord { x: 1, y: -1 },
        _ => panic!("Not recognized"),
    })
}

fn hex_str(input: &str) -> IResult<&str, &str> {
    alt((
        tag("nw"),
        tag("ne"),
        tag("e"),
        tag("se"),
        tag("sw"),
        tag("w"),
    ))(input)
}

fn hex_dir(input: &str) -> IResult<&str, HexCoord> {
    map_res(hex_str, map_dir_str)(input)
}

fn hex_dirs(input: &str) -> IResult<&str, Vec<HexCoord>> {
    many0(hex_dir)(input)
}

fn sum_coords(v: Vec<HexCoord>) -> HexCoord {
    let mut h = HexCoord { x: 0, y: 0 };
    for _v in v {
        h.x += _v.x;
        h.y += _v.y;
    }
    h
}

pub fn day24a(s: &[String]) -> i64 {
    let mut hexmap: HashMap<HexCoord, usize> = HashMap::new();
    for line in s {
        let coords = HexCoord::from_str(line);
        let sum = sum_coords(coords);
        let count = hexmap.entry(sum).or_insert(0);
        *count += 1;
    }
    let mut black_tiles = Vec::new();
    for (k, v) in hexmap {
        if v % 2 != 0 {
            black_tiles.push(k);
        }
    }
    black_tiles.len() as i64
}

impl HexCoord {
    pub fn from_str(s: &str) -> Vec<HexCoord> {
        let coords = hex_dirs(s);
        coords.unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use crate::day24;
    use crate::util;
    #[test]
    fn test_case() {
        let coords = day24::HexCoord::from_str("nwwswee");
        let coords_sum: day24::HexCoord = day24::sum_coords(coords);
        assert_eq!(coords_sum, day24::HexCoord { x: 0, y: 0 });
    }

    #[test]
    fn test_case_2() {
        let lines = util::load_strings("inputs/day24test.test");
        assert_eq!(day24::day24a(&lines), 10);
    }
}
