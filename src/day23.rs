use std::collections::VecDeque;

fn setup_game(s: &str, part: char) -> (i64, VecDeque<i64>) {
    let mut cups = VecDeque::new();
    for c in s.chars() {
        let cupnum = c.to_digit(10).unwrap() as i64;
        cups.push_back(cupnum);
    }
    if part == 'b' {
        // Add nums 10 through 1000000
        for n in 10 ..=1000000 {
            cups.push_back(n);
        }
    }
    let current_cup = &cups[0];
    (*current_cup, cups)
}

fn get_idx_of(num: i64, v: &VecDeque<i64>) -> i64 {
    for (i, _num) in v.iter().enumerate() {
        if *_num == num {
            return i as i64;
        }
    }
    -1
}

fn run_game(_current_cup: i64, cups: &mut VecDeque<i64>, part: char) -> String {
    let mut current_cup = _current_cup;
    let numsteps = match part {
        'a' => 100,
        'b' => 10000000,
        _ => panic!("Only parts a and b are allowed."),
    };
    for _ in 0..numsteps {
        //println!("Move {:?}", i + 1);
        let idx = get_idx_of(current_cup, cups);
        cups.rotate_left(idx as usize);
        //println!("cups: {:?}", cups);
        let a = cups.remove(1).unwrap();
        let b = cups.remove(1).unwrap();
        let c = cups.remove(1).unwrap();
        //println!("Pick up: {:?},{:?},{:?}", a, b, c);
        let mut nextcup = (current_cup - 1).rem_euclid(9);
        if nextcup == 0 {
            nextcup = 9;
        }
        while [a, b, c].contains(&nextcup) {
            nextcup = (nextcup - 1).rem_euclid(9);
            if nextcup == 0 {
                nextcup = 9;
            }
        }
        //println!("Destionation: {:?}", nextcup);
        let next_idx = get_idx_of(nextcup, cups);
        cups.rotate_left(next_idx as usize);
        cups.insert(1, c);
        cups.insert(1, b);
        cups.insert(1, a);
        let idx = get_idx_of(current_cup, cups);
        let next_idx = (idx + 1) % 9;
        current_cup = *cups.get(next_idx as usize).unwrap();
    }

    let oneidx = get_idx_of(1, cups);
    cups.rotate_left(oneidx as usize);
    let order: Vec<i64> = cups.range(1..cups.len()).copied().collect();
    let s: String = order.iter().map(|x| format!("{:?}", x)).collect();
    s
}

pub fn day23a(s: &str) -> i64 {
    let (i, mut cups) = setup_game(s, 'a');
    run_game(i, &mut cups, 'a').parse::<i64>().unwrap()
}

pub fn day23b(s: &str) -> i64 {
    let (i, mut cups) = setup_game(s, 'b');
    run_game(i, &mut cups, 'b').parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day23;

    #[test]
    fn test_case() {
        assert_eq!(day23::day23a("389125467"), 67384529);
    }
}
