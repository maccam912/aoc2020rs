use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
};

use regex::Regex;

struct Memtape {
    memory: HashMap<i64, i64>,
}

impl Memtape {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn sum(&self) -> i64 {
        let mut sum = 0;
        for v in self.memory.values() {
            sum += v;
        }
        sum
    }
}

impl Index<i64> for Memtape {
    type Output = i64;

    fn index(&self, idx: i64) -> &Self::Output {
        self.memory.get(&idx).unwrap_or(&0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Section {
    mask: String,
    rules: Vec<(i64, i64)>,
}

fn make_or_mask(mask: &str) -> i64 {
    let mut newchars: Vec<char> = Vec::new();
    for c in mask.chars() {
        newchars.push(match c {
            '1' => '1',
            _ => '0',
        });
    }
    let newstr: String = newchars.iter().collect();
    i64::from_str_radix(&newstr, 2).unwrap()
}

fn make_and_mask(mask: &str) -> i64 {
    let mut newchars: Vec<char> = Vec::new();
    for c in mask.chars() {
        newchars.push(match c {
            '0' => '0',
            _ => '1',
        });
    }
    let newstr: String = newchars.iter().collect();
    i64::from_str_radix(&newstr, 2).unwrap()
}

fn apply_mask_to_num(mask: &str, num: i64) -> i64 {
    let ormask = make_or_mask(mask);
    let andmask = make_and_mask(mask);
    andmask & (ormask | num)
}

fn parse_mask(line: &str) -> String {
    let re = Regex::new(r"^mask = (?P<mask>.*)$").unwrap();
    let m = re.captures(line).unwrap();
    m["mask"].to_string()
}

fn parse_rule(line: &str) -> (i64, i64) {
    let re = Regex::new(r"mem\[(?P<memloc>\d+)\] = (?P<num>\d+)").unwrap();
    let m = re.captures(line.trim()).unwrap();
    (
        m["memloc"].parse::<i64>().unwrap(),
        m["num"].parse::<i64>().unwrap(),
    )
}

fn parse_lines(lines: &[String]) -> Vec<Section> {
    let re = Regex::new(r"mask = .*").unwrap();
    let mut sections = Vec::new();
    let mut mask = String::new();
    let mut rules: Vec<(i64, i64)> = Vec::new();
    for line in lines {
        if re.is_match(line) {
            // Found match line, we have a new rule starting
            if !mask.is_empty() {
                sections.push(Section { mask, rules });
            }
            mask = parse_mask(line);
            rules = Vec::new();
        } else {
            let rule = parse_rule(line);
            rules.push(rule)
        }
    }
    if !mask.is_empty() {
        sections.push(Section { mask, rules });
    }
    sections
}

fn day14a(lines: &[String]) -> i64 {
    let sections = parse_lines(lines);
    let mut memory: Vec<i64> = vec![0; 1024 * 1024];
    for section in sections {
        let mask = section.mask;
        for rule in section.rules {
            let i = apply_mask_to_num(&mask, rule.1);
            memory[rule.0 as usize] = i;
        }
    }
    memory.iter().sum()
}

fn part_b_addresses(address: i64, mask: &str) -> Vec<i64> {
    let mut addresses = VecDeque::new();
    let s = format!("{:036b}", address);
    let mut startmask: Vec<char> = Vec::new();
    for i in 0..36 {
        if mask.chars().nth(i).unwrap() == '1' {
            startmask.push('1');
        } else if mask.chars().nth(i).unwrap() == '0' {
            match s.chars().nth(i) {
                Some(c) => startmask.push(c),
                None => startmask.push('0'),
            }
        } else if mask.chars().nth(i).unwrap() == 'X' {
            startmask.push('X');
        }
    }

    startmask.reverse();

    addresses.push_back(startmask);
    while addresses[0].contains(&'X') {
        let a = addresses.pop_front().unwrap();
        let mut loc = 0;
        for (i, c) in a.iter().enumerate() {
            if c == &'X' {
                loc = i;
            }
        }
        let mut b = a.clone();
        b[loc] = '0';
        addresses.push_back(b);
        let mut c = a.clone();
        c[loc] = '1';
        addresses.push_back(c);
    }

    addresses
        .iter()
        .map(|x| {
            let x2: String = x.iter().collect();
            x2
        })
        .map(|x: String| i64::from_str_radix(&x, 2).unwrap())
        .collect()
}

fn day14b(lines: &[String]) -> i64 {
    let sections = parse_lines(lines);
    let mut memory: Memtape = Memtape::new();
    for section in sections {
        let mask = section.mask;
        for rule in section.rules {
            let origaddress = rule.0;
            let num = rule.1;
            for address in part_b_addresses(origaddress, &mask) {
                memory.memory.insert(address, num);
            }
        }
    }
    memory.sum()
}

pub fn day14(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day14a(lines),
        'b' => day14b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day14;

    #[test]
    fn test_case() {
        let input: Vec<String> = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        assert_eq!(
            day14::parse_lines(&input),
            &[day14::Section {
                mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
                rules: vec![(8, 11), (7, 101), (8, 0)]
            }]
        );

        assert_eq!(day14::make_or_mask(&input[0]), 64);
        assert_eq!(day14::make_and_mask(&input[0]), 8796093022205);
        assert_eq!(day14::apply_mask_to_num(&input[0], 11), 73);
        assert_eq!(day14::apply_mask_to_num(&input[0], 101), 101);

        assert_eq!(day14::day14a(&input), 165);

        let input2: Vec<String> = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();
        assert_eq!(day14::day14b(&input2), 208);
    }
}
