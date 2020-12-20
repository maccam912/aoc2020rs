#![allow(dead_code)]

use num::integer::sqrt;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Tile {
    id: i64,
    field: Vec<i8>,
    width: i64,
    height: i64,
}

impl Tile {
    fn get(&self, x: usize, y: usize) -> i8 {
        let idx = (y * self.width as usize) + x;
        self.field[idx]
    }

    fn rotate(&self) -> Tile {
        let mut newfield = Vec::new();
        for x in (0..self.width as usize).rev() {
            // Go down right side, then one col left of that, etc.
            for y in 0..self.height as usize {
                newfield.push(self.get(x, y));
            }
        }
        Tile {
            id: self.id,
            field: newfield,
            width: self.width,
            height: self.height,
        }
    }
    fn flip(&self) -> Tile {
        let mut newfield = Vec::new();
        for y in 0..self.height as usize {
            for x in (0..self.width as usize).rev() {
                newfield.push(self.get(x, y));
            }
        }
        Tile {
            id: self.id,
            field: newfield,
            width: self.width,
            height: self.height,
        }
    }
    fn variants(&self) -> Vec<Tile> {
        let mut tilevariants = vec![self.clone()];
        let r1 = self.rotate();
        tilevariants.push(r1.clone());
        let r2 = r1.rotate();
        tilevariants.push(r2.clone());
        let r3 = r2.rotate();
        tilevariants.push(r3);
        let f1 = self.flip();
        tilevariants.push(f1.clone());
        let f1r1 = f1.rotate();
        tilevariants.push(f1r1.clone());
        let f1r2 = f1r1.rotate();
        tilevariants.push(f1r2.clone());
        let f1r3 = f1r2.rotate();
        tilevariants.push(f1r3);
        tilevariants
    }
    fn east(&self) -> Vec<i8> {
        let mut border = Vec::new();
        for i in 0..self.height as usize {
            border.push(self.get(self.width as usize - 1, i));
        }
        border
    }
    fn west(&self) -> Vec<i8> {
        let mut border = Vec::new();
        for i in 0..self.height as usize {
            border.push(self.get(0, i));
        }
        border
    }
    fn north(&self) -> Vec<i8> {
        let mut border = Vec::new();
        for i in 0..self.width as usize {
            border.push(self.get(i, 0));
        }
        border
    }
    fn south(&self) -> Vec<i8> {
        let mut border = Vec::new();
        for i in 0..self.width as usize {
            border.push(self.get(i, self.height as usize - 1));
        }
        border
    }
}

fn parse_tile(s: &str) -> Tile {
    let re = Regex::new(r"Tile (?P<num>\d+):").unwrap();
    let mut tileid = 0;
    let mut width: i64 = 0;
    let mut field = Vec::new();
    for (i, line) in s.split('\n').enumerate() {
        if i == 0 {
            let caps = re.captures(line).unwrap();
            tileid = caps["num"].parse::<i64>().unwrap();
            width = line.len() as i64;
        } else {
            // It's a tile line
            for (_j, c) in line.chars().enumerate() {
                let fieldnum = match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!(format!("Matched {:?}", c)),
                };
                field.push(fieldnum);
            }
        }
    }
    let height = (field.len() as i64) / width;
    Tile {
        id: tileid,
        field,
        width,
        height,
    }
}

fn split_tiles(s: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for lines in s.split("\n\n") {
        let t = parse_tile(lines);
        tiles.push(t);
    }
    tiles
}

fn get_border(tiles: &[Tile], _x: usize, y: usize, dir: char) -> Vec<i8> {
    let height_width = num::integer::sqrt(tiles.len());
    let width: usize = tiles[0].width as usize;
    let tile = &tiles[y * height_width + 1];
    match dir {
        'n' => tile.field[0..width].to_vec(),
        's' => tile.field[(width * (width - 1))..(width * width)].to_vec(),
        'e' => tile.rotate().field[0..width].to_vec(),
        'w' => tile.rotate().field[(width * (width - 1))..(width * width)].to_vec(),
        _ => panic!(),
    }
}

fn find_match(
    tiles: &[Tile],
    acc: &mut Vec<Tile>,
    idx: usize,
    puzzle_width: usize,
) -> Option<Vec<Tile>> {
    if tiles.is_empty() {
        return Some(acc.clone());
    }
    for (i, tile) in tiles.iter().enumerate() {
        for variant in tile.variants() {
            let side_is_ok = if idx % puzzle_width != 0 {
                // Check my left border against tile west
                let west_idx = idx - 1;
                let west_tile = &acc[west_idx];
                let neighbor_border = west_tile.east();
                let my_border = variant.west();
                neighbor_border == my_border
            } else {
                true
            };

            let top_is_ok = if idx >= puzzle_width {
                // Check my north border against tile north
                let north_idx = idx - puzzle_width;
                let north_tile = &acc[north_idx];
                let neighbor_border = north_tile.south();
                let my_border = variant.north();
                neighbor_border == my_border
            } else {
                true
            };
            if side_is_ok && top_is_ok {
                // This tile is fine, recurse
                let mut subtiles: Vec<Tile> = tiles.to_vec();
                subtiles.remove(i);
                let mut newacc = acc.clone();
                newacc.push(variant);
                let maybematch = find_match(&subtiles, &mut newacc, idx + 1, puzzle_width);
                if maybematch.is_some() {
                    return maybematch;
                }
            }
        }
    }
    None
}

fn solve(s: &str) -> Vec<Vec<Tile>> {
    let tiles = split_tiles(s);
    let puzzle_width = sqrt(tiles.len());
    let mut answers = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        for variant in tile.variants() {
            let mut subtiles = tiles.clone();
            subtiles.remove(i);
            let answer = find_match(&subtiles, &mut vec![variant], 1, puzzle_width);
            if let Some(a) = answer {
                answers.push(a);
            }
        }
    }
    answers
}

fn day20a(s: &str) -> i64 {
    let solutions = solve(s);
    let first_answer = &solutions[0];
    let width = sqrt(first_answer.len());
    let tl = first_answer.first().unwrap().id;
    let br = first_answer.last().unwrap().id;
    let tr = first_answer.get(width - 1).unwrap().id;
    let bl = first_answer.get(first_answer.len() - width).unwrap().id;
    tl * br * tr * bl
}

#[derive(Debug)]
struct Image {
    field: Vec<i8>,
    width: i64,
    height: i64,
}

impl Image {
    fn get(&self, x: usize, y: usize) -> i8 {
        if x as i64 >= self.width || y as i64 >= self.height {
            return 0;
        }
        let idx = y * self.width as usize + x;
        self.field[idx]
    }
}

fn squash_solution(v: &[Tile]) -> Image {
    let mut field = Vec::new();
    let tileinnerwidth = v[0].width - 2;
    let newwidth = sqrt(v.len()) as i64 * tileinnerwidth;
    for y in 0..newwidth {
        for x in 0..newwidth {
            // Get tile
            let xtile = x / tileinnerwidth;
            let ytile = y / tileinnerwidth;
            let idx = ytile * sqrt(v.len()) as i64 + xtile;
            let t: &Tile = &v[idx as usize];
            let innerx = x % tileinnerwidth;
            let innery = y % tileinnerwidth;
            let c = t.get((innerx + 1) as usize, (innery + 1) as usize);
            field.push(c);
        }
    }
    assert_eq!(field.len(), newwidth as usize * newwidth as usize);
    Image {
        field,
        width: newwidth,
        height: newwidth,
    }
}

fn check_for_monster(i: &Image, x: usize, y: usize) -> bool {
    let coordslist = [
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];
    let sum: i64 = coordslist
        .iter()
        .map(|(xx, yy)| i.get(x + xx, y + yy) as i64)
        .sum();
    sum == coordslist.len() as i64
}

fn hide_monsters(i: &mut Image) {
    let coordslist = [
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];
    for xx in 0..i.width {
        for yy in 0..i.height {
            if check_for_monster(&i, xx as usize, yy as usize) {
                // Yep monster, set those to zeros.
                for coord in &coordslist {
                    let idx = (yy + coord.1) * i.width + xx + coord.0;
                    i.field[idx as usize] = 0;
                }
            }
        }
    }
}

fn day20b(s: &str) -> i64 {
    let mut monstercounts = Vec::new();
    let solutions = solve(s);
    for solution in &solutions {
        let mut sum = 0;
        let image = squash_solution(&solution);
        for x in 0..image.width {
            for y in 0..image.height {
                if check_for_monster(&image, x as usize, y as usize) {
                    sum += 1;
                }
            }
        }
        monstercounts.push(sum);
    }
    let mut maxidx = 0;
    let mut maxmonstercount = 0;
    for (i, monstercount) in monstercounts.iter().enumerate() {
        if monstercount > &maxmonstercount {
            maxmonstercount = *monstercount;
            maxidx = i;
        }
    }
    let mut maxcountsol = squash_solution(&solutions[maxidx]);
    println!("{:?}", monstercounts);
    hide_monsters(&mut maxcountsol);
    maxcountsol.field.iter().map(|x| *x as i64).sum()
}

pub fn day20(s: &str, part: char) -> i64 {
    match part {
        'a' => day20a(s),
        'b' => day20b(s),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{day20, util};

    #[test]
    fn test_case() {
        let contents = util::load_contents("inputs/day20test.txt");
        let solutions = day20::solve(&contents);
        assert_eq!(solutions.len(), 8);

        assert_eq!(day20::day20a(&contents), 20899048083289);
        assert_eq!(day20::day20b(&contents), 273);
    }
}
