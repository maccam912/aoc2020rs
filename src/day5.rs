use std::collections::HashSet;

struct Point {
    row: i64,
    col: i64,
}

fn get_coords(line: &str) -> Point {
    let fr = &line[0..7];
    let rl = &line[7..];
    let factors: Vec<i64> = vec![64, 32, 16, 8, 4, 2, 1];
    let row: i64 = fr
        .chars()
        .map(|x| if x == 'B' { 1 } else { 0 })
        .zip(&factors)
        .map(|(x, y)| x * y)
        .sum();
    let factors2: Vec<i64> = vec![4, 2, 1];
    let col: i64 = rl
        .chars()
        .map(|x| if x == 'R' { 1 } else { 0 })
        .zip(&factors2)
        .map(|(x, y)| x * y)
        .sum();
    Point { row, col }
}

fn get_seat_id(line: &str) -> i64 {
    let p = get_coords(line);
    p.row * 8 + p.col
}

fn day5a(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| get_seat_id(line))
        .fold(0, |acc, x| if x > acc { x } else { acc })
}

fn day5b(lines: &[String]) -> i64 {
    let seat_ids: Vec<i64> = lines.iter().map(|line| get_seat_id(line)).collect();
    let min = seat_ids
        .clone()
        .into_iter()
        .fold(880, |acc, x| if x < acc { x } else { acc });
    let max = seat_ids
        .clone()
        .into_iter()
        .fold(0, |acc, x| if x > acc { x } else { acc });
    let mut seats: HashSet<i64> = HashSet::new();
    for seat_id in seat_ids.clone() {
        seats.insert(seat_id);
    }

    for id in min..max {
        if seats.get(&id).is_none() {
            return id;
        }
    }
    0
}

pub fn day5(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day5a(lines),
        'b' => day5b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day5;

    #[test]
    fn test_case() {
        assert_eq!(day5::get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(day5::get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(day5::get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(day5::get_seat_id("BBFFBBFRLL"), 820);
    }
}
