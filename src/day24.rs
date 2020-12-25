use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::many0, IResult};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

fn black_tiles(hexmap: &HashMap<HexCoord, usize>) -> Vec<HexCoord> {
    let mut black_tiles: Vec<HexCoord> = Vec::new();
    for (k, v) in hexmap {
        if v % 2 != 0 {
            black_tiles.push(k.clone());
        }
    }
    black_tiles
}

fn tile_neighbors(tile: &HexCoord) -> Vec<HexCoord> {
    let mut retval = Vec::new();
    retval.push(HexCoord {
        x: tile.x - 1,
        y: tile.y,
    });
    retval.push(HexCoord {
        x: tile.x - 1,
        y: tile.y + 1,
    });
    retval.push(HexCoord {
        x: tile.x,
        y: tile.y + 1,
    });
    retval.push(HexCoord {
        x: tile.x + 1,
        y: tile.y,
    });
    retval.push(HexCoord {
        x: tile.x + 1,
        y: tile.y - 1,
    });
    retval.push(HexCoord {
        x: tile.x,
        y: tile.y - 1,
    });
    retval
}

fn white_tiles(hexmap: &HashMap<HexCoord, usize>) -> Vec<HexCoord> {
    let blacktiles = black_tiles(hexmap);
    let mut whitetiles: Vec<HexCoord> = Vec::new();
    for tile in blacktiles {
        let neighbors = tile_neighbors(&tile);
        for neighbor in neighbors {
            if hexmap.get(&neighbor).unwrap_or(&0) % 2 == 0 {
                // Yep white, add to list
                if !whitetiles.contains(&neighbor) {
                    whitetiles.push(neighbor);
                }
            }
        }
    }
    whitetiles
}

fn step(hexmap: HashMap<HexCoord, usize>) -> HashMap<HexCoord, usize> {
    let mut next: HashMap<HexCoord, usize> = HashMap::new();
    for tile in &black_tiles(&hexmap) {
        let neighbors = tile_neighbors(&tile);
        let colors: Vec<usize> = neighbors
            .iter()
            .map(|n| hexmap.get(&n).unwrap_or(&0))
            .copied()
            .collect();
        //let numwhite = colors.iter().filter(|x| *x % 2 == 0).count();
        let numblack = colors.iter().filter(|x| *x % 2 != 0).count();
        if numblack == 0 || numblack > 2 {
            next.insert(tile.clone(), hexmap.get(tile).unwrap() + 1);
        } else {
            next.insert(tile.clone(), *hexmap.get(tile).unwrap());
        }
    }

    for tile in &white_tiles(&hexmap) {
        let neighbors = tile_neighbors(&tile);
        let colors: Vec<usize> = neighbors
            .iter()
            .map(|n| hexmap.get(&n).unwrap_or(&0))
            .copied()
            .collect();
        let numblack = colors.iter().filter(|x| *x % 2 != 0).count();
        if numblack == 2 {
            next.insert(tile.clone(), hexmap.get(tile).unwrap_or(&0) + 1);
        } else {
            next.insert(tile.clone(), *hexmap.get(tile).unwrap_or(&0));
        }
    }
    next
}

pub fn day24a(s: &[String]) -> i64 {
    let mut hexmap: HashMap<HexCoord, usize> = HashMap::new();
    for line in s {
        let coords = HexCoord::from_str(line);
        let sum = sum_coords(coords);
        let count = hexmap.entry(sum).or_insert(0);
        *count += 1;
    }
    black_tiles(&hexmap).len() as i64
}

pub fn day24b(s: &[String]) -> i64 {
    let mut hexmap: HashMap<HexCoord, usize> = HashMap::new();
    for line in s {
        let coords = HexCoord::from_str(line);
        let sum = sum_coords(coords);
        let count = hexmap.entry(sum).or_insert(0);
        *count += 1;
    }

    let mut next = hexmap;
    for _ in 0..100 {
        next = step(next);
    }
    black_tiles(&next).len() as i64
}

impl HexCoord {
    pub fn from_str(s: &str) -> Vec<HexCoord> {
        let coords = hex_dirs(s);
        coords.unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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

    #[test]
    fn test_case_3() {
        let lines = util::load_strings("inputs/day24test.test");
        assert_eq!(day24::day24b(&lines), 2208);
    }

    #[test]
    fn test_case_b() {
        let lines = util::load_strings("inputs/day24test.test");
        let mut hexmap: HashMap<day24::HexCoord, usize> = HashMap::new();
        for line in lines {
            let coords = day24::HexCoord::from_str(&line);
            let sum = day24::sum_coords(coords);
            let count = hexmap.entry(sum).or_insert(0);
            *count += 1;
        }
        assert_eq!(day24::black_tiles(&hexmap).len(), 10);
        let next = day24::step(hexmap);
        assert_eq!(day24::black_tiles(&next).len(), 15);
        let next = day24::step(next);
        assert_eq!(day24::black_tiles(&next).len(), 12);
        let next = day24::step(next);
        assert_eq!(day24::black_tiles(&next).len(), 25);
        let next = day24::step(next);
        assert_eq!(day24::black_tiles(&next).len(), 14);
    }
}
