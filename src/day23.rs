use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

const MY_INPUT: &str = "643719258";

fn setup_game(s: &str) -> (u8, VecDeque<u8>) {
    let mut cups = VecDeque::new();
    for c in s.chars() {
        let cupnum = c.to_digit(10).unwrap() as u8;
        cups.push_back(cupnum);
    }

    let current_cup = &cups[0];
    (*current_cup, cups)
}

fn get_idx_of(num: u8, v: &VecDeque<u8>) -> usize {
    for (i, _num) in v.iter().enumerate() {
        if *_num == num {
            return i;
        }
    }
    99
}

fn run_game(_current_cup: u8, cups: &mut VecDeque<u8>) {
    let mut current_cup = _current_cup;
    for _ in 0..100 {
        let idx = get_idx_of(current_cup, cups);
        println!("1: Length {:?} mid {:?}", cups.len(), idx);
        cups.rotate_left(idx);
        let a = cups.remove(1).unwrap();
        let b = cups.remove(1).unwrap();
        let c = cups.remove(1).unwrap();
        let mut nextcup = (current_cup-1).rem_euclid(9);
        while [a, b, c].contains(&nextcup) {
            nextcup = (nextcup-1).rem_euclid(9);
        }
        if nextcup == 0 {
            nextcup = 9;
        }
        let next_idx = get_idx_of(nextcup, cups);
        println!("2: Length {:?} mid {:?}", cups.len(), next_idx);
        cups.rotate_left(next_idx);
        cups.insert(1, a);
        cups.insert(1, b);
        cups.insert(1, c);
        let idx = get_idx_of(current_cup, cups);
        let next_idx = (idx+1) % 9;
        current_cup = *cups.get(next_idx).unwrap();
    }

    let oneidx = get_idx_of(1, cups);
    println!("3: Length {:?} mid {:?}", cups.len(), oneidx);
    cups.rotate_left(oneidx);
    let order: Vec<u8> = cups.range(1..cups.len()).copied().collect();
    println!("{:?}", order);
}

fn day23a() -> i64 {
    let (i, mut cups) = setup_game(MY_INPUT);
    run_game(i, &mut cups);
    0
}

#[cfg(test)]
mod tests {
    use crate::day23;

    #[test]
    fn test_case() {
        day23::day23a();
    }
}