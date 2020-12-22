use std::collections::{HashSet, VecDeque};

fn parse_contents(s: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut decks = Vec::new();
    for player in s.split("\n\n") {
        // drop first line, thats player number
        let mut lines = player.split('\n');
        lines.next();
        let mut deck = VecDeque::new();
        for line in lines {
            let num = line.trim().parse::<usize>().unwrap();
            deck.push_back(num);
        }
        decks.push(deck);
    }

    (decks[0].clone(), decks[1].clone())
}

fn step_a(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    let p1card = p1.front().unwrap();
    let p2card = p2.front().unwrap();
    match p1card.cmp(p2card) {
        std::cmp::Ordering::Greater => {
            p1.rotate_left(1);
            p1.push_back(p2.pop_front().unwrap());
        }
        std::cmp::Ordering::Less => {
            p2.rotate_left(1);
            p2.push_back(p1.pop_front().unwrap());
        }
        std::cmp::Ordering::Equal => panic!("There was a tie? Impossible!"),
    }
}

fn step_b(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, depth: usize) {
    let p1card = p1.front().unwrap();
    let p2card = p2.front().unwrap();
    if p1.len() > *p1card && p2.len() > *p2card {
        // Recurse into subgame
        let mut newp1cards: VecDeque<usize> = p1.range(1..1 + p1card).copied().collect();

        let mut newp2cards: VecDeque<usize> = p2.range(1..1 + p2card).copied().collect();
        let (winner, _winningdeck) = run_game_b(&mut newp1cards, &mut newp2cards, depth + 1);
        match winner {
            '1' => {
                p1.rotate_left(1);
                p1.push_back(p2.pop_front().unwrap());
            }
            '2' => {
                p2.rotate_left(1);
                p2.push_back(p1.pop_front().unwrap());
            }
            _ => panic!("Someone other than p1 or p2 won??"),
        };
    } else if p1card > p2card {
        p1.rotate_left(1);
        p1.push_back(p2.pop_front().unwrap());
    } else if p2card > p1card {
        p2.rotate_left(1);
        p2.push_back(p1.pop_front().unwrap());
    } else {
        panic!("It's a tie!");
    }
}

fn run_game_a(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> VecDeque<usize> {
    while !p1.is_empty() && !p2.is_empty() {
        step_a(p1, p2);
    }

    if !p1.is_empty() {
        p1.clone()
    } else {
        p2.clone()
    }
}

fn run_game_b(
    p1: &mut VecDeque<usize>,
    p2: &mut VecDeque<usize>,
    depth: usize,
) -> (char, VecDeque<usize>) {
    let mut prev_states = HashSet::<(VecDeque<usize>, VecDeque<usize>)>::default();

    while !p1.is_empty() && !p2.is_empty() {
        if prev_states.contains(&(p1.clone(), p2.clone())) {
            // We've seen this state. P1 wins!
            return ('1', p1.clone());
        } else {
            prev_states.insert((p1.clone(), p2.clone()));
            step_b(p1, p2, depth);
        }
    }

    if !p1.is_empty() {
        ('1', p1.clone())
    } else {
        ('2', p2.clone())
    }
}

fn score_winner(deck: &VecDeque<usize>) -> usize {
    let mut sum = 0;
    for (idx, cardpos) in (0..deck.len()).rev().enumerate() {
        let card = deck[cardpos];
        let multiplier = idx + 1;
        sum += multiplier * card;
    }
    sum
}

fn day22a(contents: &str) -> i64 {
    let (mut p1, mut p2) = parse_contents(contents);
    let winner = run_game_a(&mut p1, &mut p2);
    score_winner(&winner) as i64
}

fn day22b(contents: &str) -> i64 {
    let (mut p1, mut p2) = parse_contents(contents);
    let (_, winner) = run_game_b(&mut p1, &mut p2, 0);
    score_winner(&winner) as i64
}

pub fn day22(contents: &str, part: char) -> i64 {
    match part {
        'a' => day22a(contents),
        'b' => day22b(contents),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day22;
    use crate::util;

    #[test]
    fn test_case_1() {
        let contents = util::load_contents("inputs/day22test.test");
        let ans_a = day22::day22(&contents, 'a');
        assert_eq!(ans_a, 306);
    }

    #[test]
    fn test_case_2() {
        let contents = util::load_contents("inputs/day22test.test");
        let ans_b = day22::day22(&contents, 'b');
        assert_eq!(ans_b, 291);
    }
}
