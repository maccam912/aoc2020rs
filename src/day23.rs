fn setup_game(s: &str, part: char) -> (usize, Vec<usize>) {
    let cupslen = match part {
        'a' => s.len(),
        'b' => 1000000,
        _ => panic!("Only parts a and b are allowed"),
    };
    let mut cups: Vec<usize> = vec![0; cupslen + 1];
    let chars: Vec<char> = s.chars().collect();
    for i in 1..s.len() {
        let cupnum = chars[i - 1].to_digit(10).unwrap();
        let nextcup = chars[i].to_digit(10).unwrap();
        cups[cupnum as usize] = nextcup as usize;
    }
    if part == 'a' {
        let cupnum = chars[s.len() - 1].to_digit(10).unwrap();
        let nextcup = chars[0].to_digit(10).unwrap();
        cups[cupnum as usize] = nextcup as usize;
    }
    if part == 'b' {
        let cupnum = chars[s.len() - 1].to_digit(10).unwrap();
        cups[cupnum as usize] = 10;
        // Add nums 10 through 1000000
        for (n, cup) in cups.iter_mut().enumerate().take(1000000).skip(10) {
            *cup = (n + 1) as usize;
        }
        cups[1000000] = chars[0].to_digit(10).unwrap() as usize;
    }
    let current_cup = s.chars().next().unwrap().to_digit(10).unwrap();
    (current_cup as usize, cups)
}

// fn print_cups(orig_cup: usize, cups: &[usize]) -> String {
//     let mut current_cup = orig_cup;
//     let mut strings = Vec::new();
//     let mut count = 0;
//     while cups[current_cup] != orig_cup && count < 12 {
//         strings.push(format!("{:?}", current_cup));
//         current_cup = cups[current_cup];
//         count += 1;
//     }
//     strings.push(format!("{:?}", current_cup));
//     strings.join(" ")
// }

fn run_game(_current_cup: usize, cups: &mut Vec<usize>, part: char) -> String {
    let mut current_cup = _current_cup;
    let numsteps = match part {
        'a' => 100,
        'b' => 10000000,
        _ => panic!("Only parts a and b are allowed."),
    };
    for _ in 0..numsteps {
        // Get next three after current cup
        let a = cups[current_cup];
        let b = cups[a];
        let c = cups[b];
        // Get one after C. current_cup should point to there next
        let next = cups[c];
        cups[current_cup] = next;

        let mut destination_cup = current_cup.checked_sub(1).unwrap();
        if destination_cup == 0 {
            destination_cup = match part {
                'a' => 9,
                'b' => 1000000,
                _ => panic!("Only part a or b allowed"),
            };
        }
        while [a, b, c].contains(&destination_cup) {
            destination_cup = destination_cup.checked_sub(1).unwrap();
            if destination_cup == 0 {
                destination_cup = match part {
                    'a' => 9,
                    'b' => 1000000,
                    _ => panic!("Only part a or b allowed"),
                };
            }
        }
        // destination_cup now is our destination, put abc in there
        let cnext = cups[destination_cup];
        cups[destination_cup] = a;
        // These didn't change
        // cups[a] = b;
        // cups[b] = c;
        cups[c] = cnext;
        current_cup = cups[current_cup];
    }

    match part {
        'a' => {
            // Get everything after 1, put it into a string.
            let mut n = cups[1];
            let mut s = String::new();
            while n != 1 {
                s.push_str(&format!("{:?}", n));
                n = cups[n];
            }
            s
        }
        'b' => {
            let a = cups[1];
            let b = cups[a];
            format!("{:?}", a * b)
        }
        _ => panic!("Only parts a and b are allowed"),
    }
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
    fn test_case_a() {
        assert_eq!(day23::day23a("389125467"), 67384529);
    }

    #[test]
    fn test_case_b() {
        assert_eq!(day23::day23b("389125467"), 149245887792);
    }
}
