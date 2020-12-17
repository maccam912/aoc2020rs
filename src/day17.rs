use crate::infinite_field_3d::InfiniteField3d;

fn next_step(field: &InfiniteField3d<i64>) -> InfiniteField3d<i64> {
    let mut newfield: InfiniteField3d<i64> = InfiniteField3d::new();
    for x in field.xlims.0 - 1..=field.xlims.1 + 1 {
        for y in field.ylims.0 - 1..=field.ylims.1 + 1 {
            for z in field.zlims.0 - 1..=field.zlims.1 + 1 {
                let num_neighbors = field.num_neighbors(x as isize, y as isize, z as isize);
                if num_neighbors == 3 {
                    newfield.set(x as isize, y as isize, z as isize, 1);
                } else if (2..=3).contains(&num_neighbors) {
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

fn parse_input(input: &str) -> InfiniteField3d<i64> {
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

fn day17a(input: &str) -> i64 {
    let mut field = parse_input(&input);
    for _ in 0..6 {
        field = next_step(&field);
    }
    field.sum()
}

pub fn day17(input: &str, part: char) -> i64 {
    match part {
        'a' => day17a(input),
        //'b' => day17b(input),
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
        let mut field = day17::parse_input(&input);
        assert_eq!(field.sum(), 5);
        for i in 0..6 {
            field = day17::next_step(&field);
            println!("Iteration {:?}: {:?}", i, field.sum());
            for quad in &field.quads {
                let sum: i64 = quad.field.iter().sum();
            }
        }
        println!("{}", field);
        assert_eq!(field.sum(), 112);
    }
}
