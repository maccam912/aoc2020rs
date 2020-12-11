use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Floor,
    Free,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
struct WorldCell {
    state: CellState,
}

#[derive(Debug)]
struct World {
    field: Vec<WorldCell>,
    width: usize,
    height: usize,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars: Vec<char> = Vec::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = (row * self.width) + col;
                let c = self.field.get(idx).unwrap();
                chars.push(match &c.state {
                    CellState::Floor => '.',
                    CellState::Free => 'L',
                    CellState::Occupied => '#',
                });
            }
            chars.push('\n');
        }

        let displaystr: String = chars.iter().collect();
        write!(f, "{}", displaystr)
    }
}

impl World {
    fn get_cell(&self, x: i64, y: i64) -> Option<&WorldCell> {
        if x < 0 || x >= self.width as i64 {
            return None;
        }

        if y < 0 || y >= self.height as i64 {
            return None;
        }

        let idx = (y * (self.width as i64)) + x;
        if idx < 0 || idx >= (self.field.len() as i64) {
            return None;
        }

        self.field.get(idx as usize)
    }

    fn get_neighbors(&self, x: usize, y: usize, part: char) -> Vec<WorldCell> {
        let mut neighbors = Vec::new();
        for dir in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let maxdist: i64 = match part {
                'a' => 1,
                'b' => (self.width + self.height) as i64,
                _ => panic!(),
            };
            for dist in 1..=maxdist {
                // overkill on max distance, but we need to at least go far enough
                let offset: (i64, i64) = (dir.0 * dist, dir.1 * dist);
                //println!("Point {:?},{:?}: Dist {:?}, offset {:?}", x, y, dist, offset);
                let cell_in_question = ((x as i64) + offset.0, (y as i64) + offset.1);
                if let Some(c) = self.get_cell(cell_in_question.0, cell_in_question.1) {
                    if c.state == CellState::Occupied {
                        neighbors.push(c.clone());
                    }
                }
            }
        }
        neighbors
    }

    fn old_get_neighbors(&self, x: usize, y: usize, part: char) -> Vec<&WorldCell> {
        let mut neighbors = Vec::new();
        for xx in (x as i64) - 1..=(x as i64) + 1 {
            let xdir = xx - (x as i64);
            for yy in (y as i64) - 1..=(y as i64) + 1 {
                let ydir = yy - (y as i64);
                if (0..self.width).contains(&(xx as usize))
                    && (0..self.height).contains(&(yy as usize))
                    && !(xx == (x as i64) && yy == (y as i64))
                {
                    let mut cell = self.get_cell(xx, yy);
                    if let Some(c) = cell {
                        if c.state == CellState::Occupied {
                            neighbors.push(c);
                        }
                    }
                    if part == 'b' && cell.is_some() && cell.unwrap().state != CellState::Occupied {
                        let mut newx = (x as i64) + xdir;
                        let mut newy = (y as i64) + ydir;
                        cell = self.get_cell(newx, newy);
                        while cell.is_some() {
                            if cell.unwrap().state == CellState::Occupied {
                                neighbors.push(cell.unwrap());
                                break;
                            }
                            newx += xdir;
                            newy += ydir;
                            println!("{}/{}, {},{}: {} {}", x, y, xdir, ydir, newx, newy);
                            // Get cell one further away
                            cell = self.get_cell(newx, newy);
                        }
                    }
                }
            }
        }
        println!("{:?}", neighbors);
        neighbors
    }

    fn step(&mut self, part: char) {
        let mut newfield = Vec::new();
        let neighborthresh = match part {
            'a' => 4,
            'b' => 5,
            _ => panic!(),
        };
        for row in 0..self.height {
            for col in 0..self.width {
                let n = self.get_neighbors(col, row, part);
                let occupiedcount = n.iter().filter(|x| x.state == CellState::Occupied).count();
                let currentstate = self.get_cell(col as i64, row as i64).unwrap();
                if occupiedcount == 0 && currentstate.state == CellState::Free {
                    newfield.push(WorldCell {
                        state: CellState::Occupied,
                    });
                } else if occupiedcount >= neighborthresh
                    && currentstate.state == CellState::Occupied
                {
                    newfield.push(WorldCell {
                        state: CellState::Free,
                    })
                } else {
                    newfield.push(WorldCell {
                        state: currentstate.state,
                    })
                }
            }
        }
        self.field = newfield;
    }

    fn run(&mut self, part: char) {
        loop {
            let currstate: &[WorldCell] = &self.field.clone();
            self.step(part);
            println!("\n{}\n", self);
            let newstate: &[WorldCell] = &self.field;

            if currstate == newstate {
                break;
            }
        }
    }

    fn count_occupied(&self) -> i64 {
        self.field
            .iter()
            .filter(|x| x.state == CellState::Occupied)
            .count() as i64
    }
}

fn make_world(lines: &[String]) -> World {
    let height = lines.len();
    let width = lines[0].trim().len();
    let mut field = Vec::new();
    for line in lines.iter() {
        for c in line.trim().chars() {
            let w = WorldCell {
                state: match c {
                    '.' => CellState::Floor,
                    'L' => CellState::Free,
                    '#' => CellState::Occupied,
                    _ => panic!("Character {} is not recognized", c),
                },
            };
            field.push(w);
        }
    }
    World {
        field,
        width,
        height,
    }
}

fn day11a(lines: &[String]) -> i64 {
    let mut world = make_world(&lines);
    world.run('a');
    world.count_occupied()
}

fn day11b(lines: &[String]) -> i64 {
    let mut world = make_world(&lines);
    world.run('b');
    world.count_occupied()
}

pub fn day11(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day11a(lines),
        'b' => day11b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day11;

    #[test]
    fn test_case() {
        let input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

        let input2 = "LL
        LL
        LL
        LL
        LL
        LL
        LL
        LL
        LL
        LL";

        let lines: Vec<String> = input.split('\n').map(|x| x.to_string()).collect();
        let lines2: Vec<String> = input2.split('\n').map(|x| x.to_string()).collect();

        let mut world = day11::make_world(&lines);
        let world2 = day11::make_world(&lines2);

        assert!(world2.get_cell(1, 5).is_some());
        assert!(world2.get_cell(5, 1).is_none());

        let neighbors = world.get_neighbors(0, 0, 'a');
        assert_eq!(neighbors.len(), 0);
        let neighbors2 = world.get_neighbors(5, 8, 'a');
        for n in neighbors2 {
            assert_eq!(
                n,
                day11::WorldCell {
                    state: day11::CellState::Free
                }
            );
        }

        world.step('a');
        let neighbors = world.get_neighbors(0, 0, 'a');
        assert_eq!(
            neighbors[0],
            day11::WorldCell {
                state: day11::CellState::Occupied
            }
        );
        let neighbors2 = world.get_neighbors(5, 8, 'a');
        for n in neighbors2 {
            assert_eq!(
                n,
                day11::WorldCell {
                    state: day11::CellState::Occupied
                }
            );
        }

        assert_eq!(day11::day11a(&lines), 37);
        assert_eq!(day11::day11a(&lines2), 12);

        //let mut world1again = day11::make_world(&lines);
        assert_eq!(day11::day11b(&lines), 26);
    }
}
