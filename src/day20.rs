use regex::Regex;

#[derive(Debug, Clone)]
struct Tile {
    id: i64,
    field: Vec<i8>,
    width: i64,
    height: i64,
}

impl Tile {
    fn get(&self, x: usize, y: usize) -> i8 {
        let idx = (y*self.width as usize)+x;
        self.field[idx]
    }

    fn rotate(&self) -> Tile {
        let mut newfield = Vec::new();
        for x in (0..self.width as usize).rev() { // Go down right side, then one col left of that, etc.
            for y in 0..self.height as usize {
                newfield.push(self.get(x, y));
            }
        }
        Tile {id: self.id, field: newfield, width: self.width, height: self.height }
    }
    fn flip(&self) -> Tile {
        let mut newfield = Vec::new();
        for y in 0..self.height as usize {
            for x in (0..self.width as usize).rev() {
                newfield.push(self.get(x, y));
            }
        }
        Tile {id: self.id, field: newfield, width: self.width, height: self.height }
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
    let height = (field.len() as i64)/width;
    let t = Tile { id: tileid,  field, width, height };
    t
}

fn split_tiles(s: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for lines in s.split("\n\n") {
        let t = parse_tile(lines);
        tiles.push(t);
    }
    tiles
}

fn id_int_str(id: i64, with_variants: bool) -> String {
    match with_variants {
        false => format!("{:?}", id),
        true => {
            let orig = id*10;
            format!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", orig, orig+1, orig+2, orig+3, orig+4, orig+5, orig+6, orig+7)
        }
    }
}

fn id_int_set(tiles: &[Tile], with_variants: bool) -> String {
    let ints = tiles.into_iter().map(|x| id_int_str(x.id, with_variants)).collect::<Vec<String>>().join(", ");
    format!("set of int: ID = {{ {} }};", ints)
}

fn tiles_line(tiles: &[Tile]) -> String {
    let height_width = num::integer::sqrt(tiles.len());
    format!("array[0..{:?},0..{:?}] of var ID: tiles; % tile at point (i,j)", height_width, height_width)
}

fn get_identifiers(tiles: &[Tile]) -> String {
    let height_width = num::integer::sqrt(tiles.len());
    let mut strs = Vec::new();
    for x in 0..height_width {
        for y in 0..height_width {
            for dir in "nsew".chars() {
                let tileborder = format!("{}_{:?}_{:?}", dir, x, y);
                let varname = format!("{}borders", match dir {
                    'n' => "north",
                    'e' => "east",
                    'w' => "west",
                    's' => "south",
                    _ => panic!(),
                });
                let str = format!("var int: {};", tileborder);
                strs.push(str);
            }
        }
    }
    strs.join("\n")
}

fn get_border(tiles: &[Tile], x: usize, y: usize, dir: char) -> Vec<i8> {
    let height_width = num::integer::sqrt(tiles.len());
    let width: usize = tiles[0].width as usize;
    let tile = &tiles[y*height_width+1];
    match dir {
        'n' => tile.field[0..width].to_vec(),
        's' => tile.field[(width*(width-1))..(width*width)].to_vec(),
        'e' => tile.rotate().field[0..width].to_vec(),
        'w' => tile.rotate().field[(width*(width-1))..(width*width)].to_vec(),
        _ => panic!(),
    }
}

fn border_set(tiles: &[Tile], dir: char) -> String {
    let height_width = num::integer::sqrt(tiles.len());
    let mut borderstrs = Vec::new();
    for tilex in 0..height_width {
        for tiley  in 0..height_width {
            let border = get_border(tiles, tilex, tiley, dir);
            let borderstr = format!("\"[{}]\"", border.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", "));
            borderstrs.push(borderstr);
        }
    }

    let varname = format!("{}borders", match dir {
        'n' => "north",
        'e' => "east",
        's' => "south",
        'w' => "west",
        _ => panic!(),
    });
    format!("set of string: {} = {{ {} }};", varname, borderstrs.join(", "))
}

fn border_constraints(tiles: &[Tile]) -> String {
    let height_width = num::integer::sqrt(tiles.len());
    let mut constraints_lines = Vec::new();
    for tilex in 0..height_width {
        for tiley in 0..height_width {
            let my_right = format!("e_{:?}_{:?}", tilex, tiley);
            let my_bottom = format!("s_{:?}_{:?}", tilex, tiley);
            let neighbors_left = format!("w_{:?}_{:?}", tilex+1, tiley);
            let neighbors_top = format!("n_{:?}_{:?}", tilex, tiley+1);
            if tilex < height_width-1 {
                let line1 = format!("constraint eastborders[{}] == westborders[{}];", my_right, neighbors_left);
                constraints_lines.push(line1);
            }
            if tiley < height_width-1 {
                let line2 = format!("constraint southborders[{}] == northborders[{}];", my_bottom, neighbors_top);
                constraints_lines.push(line2);
            }
        }
    }
    constraints_lines.join("\n")
}

fn imports() -> String {
    "include \"alldifferent.mzn\";".to_string()
}

fn minizinc_prog(tiles: &[Tile]) -> String {
    let strs: Vec<String> = vec![
        imports(),
        border_set(tiles, 'n'),
        border_set(tiles, 'e'),
        border_set(tiles, 's'),
        border_set(tiles, 'w'),
        get_identifiers(tiles),
        border_constraints(tiles),
        id_int_set(tiles, true),
        tiles_line(tiles),
        "constraint alldifferent(tiles);".to_string(),
    ];
    strs.join("\n\n")
}

#[cfg(test)]
mod tests {
    use crate::{day20, util};

    #[test]
    fn test_case() {
        let contents = util::load_contents("inputs/day20test.txt");
        let tiles = day20::split_tiles(&contents);
        println!("{}", day20::minizinc_prog(&tiles));
        assert_eq!(1, 2);
        let variant_tiles: Vec<day20::Tile> = tiles.iter().flat_map(|t| t.variants()).collect();
        assert_eq!(variant_tiles.len(), tiles.len()*8);
        assert_eq!(variant_tiles[0].get(1, 1), variant_tiles[1].get(1, 8));
        assert_eq!(variant_tiles[0].get(1, 1), variant_tiles[4].get(8, 1));
    }
}
