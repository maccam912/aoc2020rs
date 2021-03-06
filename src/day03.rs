#[derive(Debug)]
struct Field {
    data: Vec<Vec<usize>>,
    width: i64,
    height: i64,
}

impl Field {
    fn get(&self, x: i64, y: i64) -> Option<usize> {
        let row: &Vec<usize> = self.data.get((y as usize) - 1)?;
        let val: &usize = row.get(((x as usize) - 1) % self.width as usize)?;
        Some(*val)
    }

    fn count_path(&self, x: i64, y: i64) -> Option<usize> {
        let mut currx = 1;
        let mut curry = 1;
        let mut sum: usize = 0;
        while curry < self.height + 1 {
            sum += self.get(currx, curry)?;
            currx += x;
            curry += y;
        }
        Some(sum)
    }
}

fn convert_input_to_array(input: &[String]) -> Field {
    let height: i64 = input.len() as i64;
    let width: i64 = input[0].len() as i64;
    let vecs: Vec<Vec<usize>> = input
        .iter()
        .map(|row| row.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    Field {
        data: vecs,
        width,
        height,
    }
}

fn day03a(lines: &[String]) -> usize {
    let field = convert_input_to_array(lines);
    field.count_path(3, 1).unwrap()
}

fn day03b(lines: &[String]) -> usize {
    let field = convert_input_to_array(lines);
    let mut prod = 1;
    prod *= field.count_path(1, 1).unwrap();
    prod *= field.count_path(3, 1).unwrap();
    prod *= field.count_path(5, 1).unwrap();
    prod *= field.count_path(7, 1).unwrap();
    prod *= field.count_path(1, 2).unwrap();
    prod
}

pub fn day03(lines: &[String], part: char) -> usize {
    match part {
        'a' => day03a(lines),
        'b' => day03b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day03;

    #[test]
    fn test_case() {
        let raw_test_input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        let lines = raw_test_input.split("\n");
        let test_input: Vec<String> = lines.map(|x| x.trim().to_string()).collect();

        let field: day03::Field = day03::convert_input_to_array(&test_input);
        assert_eq!(field.get(1, 1), Some(0));
        assert_eq!(field.get(1, 2), Some(1));
        assert_eq!(field.get(12, 2), Some(1));

        assert_eq!(field.count_path(3, 1), Some(7));

        assert_eq!(day03::day03a(&test_input), 7);
        assert_eq!(day03::day03b(&test_input), 336);
    }
}
