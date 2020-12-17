use crate::{infinite_field_3d::InfiniteField3d, infinite_field_4d::InfiniteField4d};

fn next_step_a(field: &InfiniteField3d<i64>) -> InfiniteField3d<i64> {
    let mut newfield: InfiniteField3d<i64> = InfiniteField3d::new();
    for x in field.xlims.0 - 1..=field.xlims.1 + 1 {
        for y in field.ylims.0 - 1..=field.ylims.1 + 1 {
            for z in field.zlims.0 - 1..=field.zlims.1 + 1 {
                let num_neighbors = field.num_neighbors(x as isize, y as isize, z as isize);
                if num_neighbors == 3 {
                    newfield.set(x as isize, y as isize, z as isize, 1);
                } else if !(2..=3).contains(&num_neighbors) {
                    if field.get(x as isize, y as isize, z as isize) == 1 {
                        newfield.set(x as isize, y as isize, z as isize, 0);
                    }
                } else {
                    newfield.set(
                        x as isize,
                        y as isize,
                        z as isize,
                        field.get(x as isize, y as isize, z as isize),
                    );
                }
            }
        }
    }
    newfield
}

fn next_step_b(field: &InfiniteField4d<i64>) -> InfiniteField4d<i64> {
    let mut newfield: InfiniteField4d<i64> = InfiniteField4d::new();
    for x in field.xlims.0 - 1..=field.xlims.1 + 1 {
        for y in field.ylims.0 - 1..=field.ylims.1 + 1 {
            for z in field.zlims.0 - 1..=field.zlims.1 + 1 {
                for w in field.wlims.0 - 1..=field.wlims.1 + 1 {
                    let num_neighbors =
                        field.num_neighbors(x as isize, y as isize, z as isize, w as isize);
                    if num_neighbors == 3 {
                        newfield.set(x as isize, y as isize, z as isize, w as isize, 1);
                    } else if !(2..=3).contains(&num_neighbors) {
                        if field.get(x as isize, y as isize, z as isize, w as isize) == 1 {
                            newfield.set(x as isize, y as isize, z as isize, w as isize, 0);
                        }
                    } else {
                        newfield.set(
                            x as isize,
                            y as isize,
                            z as isize,
                            w as isize,
                            field.get(x as isize, y as isize, z as isize, w as isize),
                        );
                    }
                }
            }
        }
    }
    newfield
}

fn parse_input_a(input: &str) -> InfiniteField3d<i64> {
    let mut field = InfiniteField3d::new();
    for (lnum, line) in input.split('\n').enumerate() {
        for (cnum, c) in line.trim().chars().enumerate() {
            let vnum = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!(),
            };
            field.set(cnum as isize, lnum as isize, 0, vnum);
        }
    }
    field
}

fn parse_input_b(input: &str) -> InfiniteField4d<i64> {
    let mut field = InfiniteField4d::new();
    for (lnum, line) in input.split('\n').enumerate() {
        for (cnum, c) in line.trim().chars().enumerate() {
            let vnum = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!(),
            };
            field.set(cnum as isize, lnum as isize, 0, 0, vnum);
        }
    }
    field
}

fn day17a(input: &str) -> i64 {
    let mut field = parse_input_a(&input);
    for _ in 0..6 {
        field = next_step_a(&field);
    }
    field.sum()
}

fn day17b(input: &str) -> i64 {
    let mut field = parse_input_b(&input);
    for _ in 0..6 {
        field = next_step_b(&field);
    }
    field.sum()
}

pub fn day17(input: &str, part: char) -> i64 {
    match part {
        'a' => day17a(input),
        'b' => day17b(input),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day17;

    #[test]
    fn test_case() {
        let input = ".#.
        ..#
        ###"
        .to_string();
        let mut field = day17::parse_input_a(&input);
        assert_eq!(field.sum(), 5);
        for i in 0..6 {
            field = day17::next_step_a(&field);
            println!("Iteration {:?}: {:?}", i, field.sum());
        }
        println!("{}", field);
        assert_eq!(field.sum(), 112);

        let mut field = day17::parse_input_b(&input);
        assert_eq!(field.sum(), 5);
        for i in 0..6 {
            field = day17::next_step_b(&field);
            println!("Iteration {:?}: {:?}", i, field.sum());
        }
        //println!("{}", field);
        assert_eq!(field.sum(), 848);
    }
}
