use std::{
    cmp::Ordering,
    collections::HashSet,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Clone)]
struct Range {
    lo: i64,
    hi: i64,
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    ranges: Vec<Range>,
}

impl Rule {
    fn range_size(&self) -> usize {
        self.ranges.iter().map(|x| (1 + x.hi - x.lo) as usize).sum()
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range_size().cmp(&other.range_size())
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Rule {}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    nums: Vec<i64>,
}

#[derive(Debug, PartialEq)]
struct Day {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_rules(s: &str) -> Vec<Rule> {
    let lines = s.split('\n');
    let mut rulesvec = Vec::new();
    for line in lines {
        let mut sections = line.trim().split(':');
        let name: String = sections.next().unwrap().to_string();
        let ranges = sections.next().unwrap().split(" or ");
        let mut rangelist = Vec::new();
        for range in ranges {
            let mut lohi = range.trim().split('-');
            let lo = lohi.next().unwrap();
            let hi = lohi.next().unwrap();
            let r = Range {
                lo: lo.parse::<i64>().unwrap(),
                hi: hi.parse::<i64>().unwrap(),
            };
            rangelist.push(r);
        }
        rulesvec.push(Rule {
            name,
            ranges: rangelist,
        });
    }
    rulesvec
}

fn parse_ticket(s: &str) -> Ticket {
    Ticket {
        nums: s
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    }
}

fn parse_input(s: &str) -> Day {
    let sections: Vec<String> = s.split("\n\n").map(|x| x.to_string()).collect();
    let rules = parse_rules(&sections[0]);
    let my_ticket = parse_ticket(sections[1].split('\n').nth(1).unwrap());
    let mut nearby_tickets_lines: Vec<&str> = sections[2].split('\n').collect::<Vec<&str>>();
    nearby_tickets_lines.remove(0);
    let mut nearby_tickets = Vec::new();
    for line in nearby_tickets_lines {
        let nearby_ticket = parse_ticket(line);
        nearby_tickets.push(nearby_ticket);
    }
    Day {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn check_num(day: &Day, num: i64) -> bool {
    let mut valids = Vec::new();
    for rule in &day.rules {
        let mut in_range = Vec::new();
        for range in &rule.ranges {
            in_range.push(num >= range.lo && num <= range.hi);
        }
        let valid_num = in_range.iter().any(|b| *b);
        valids.push(valid_num);
    }
    valids.iter().any(|b| *b)
}

fn day16a(input: &str) -> i64 {
    let day = parse_input(input);
    let mut invalid_nums = Vec::new();
    for other_ticket in &day.nearby_tickets {
        for num in &other_ticket.nums {
            if !check_num(&day, *num) {
                invalid_nums.push(num);
            }
        }
    }
    invalid_nums.iter().copied().sum()
}

fn is_valid(rule: &Rule, nums: &[i64]) -> bool {
    for num in nums {
        let mut valid = false;
        for range in &rule.ranges {
            if num >= &range.lo && num <= &range.hi {
                valid = true;
                break;
            }
        }
        if !valid {
            return false;
        }
    }
    true
}

fn step(rules: HashSet<Rule>, numlists: Vec<Vec<i64>>) -> Option<Vec<String>> {
    let mut sortedrules: Vec<Rule> = rules.iter().cloned().collect();
    sortedrules.sort();
    sortedrules.reverse();
    for rule in &sortedrules {
        if rules.len() == 20 {
            println!("Rule {:?}", rule.name);
        }
        if rules.len() == 19 {
            println!("  Rule {:?}", rule.name);
        }
        if rules.len() == 18 {
            println!("    Rule {:?}", rule.name);
        }
        if rules.len() == 17 {
            println!("      Rule {:?}", rule.name);
        }
        let nums = &numlists[0];
        if is_valid(rule, &nums) {
            // this rule is valid, remove it from the list, remove this column of nums, try next rule
            let mut newrules = rules.clone();
            newrules.remove(rule);
            if newrules.is_empty() {
                return Some(vec![rule.name.clone()]);
            }
            let result = step(newrules, numlists[1..].to_vec());
            if let Some(r) = result {
                let mut newvec: Vec<String> = vec![rule.name.clone()];
                let mut prevvec = r;
                newvec.append(&mut prevvec);
                return Some(newvec);
            }
        }
    }
    // Uh oh, we made it this far. No solution. return None
    None
}

fn is_valid_ticket(day: &Day, ticket: &Ticket) -> bool {
    ticket.nums.iter().all(|x| check_num(day, *x))
}

fn order_of_answers(input: &str) -> Vec<String> {
    let mut day = parse_input(input);
    day.nearby_tickets = day
        .nearby_tickets
        .iter()
        .filter(|x| is_valid_ticket(&day, x))
        .cloned()
        .collect();
    let mut numlists: Vec<Vec<i64>> = Vec::new();
    for i in 0..day.my_ticket.nums.len() {
        let numlist: Vec<i64> = day
            .nearby_tickets
            .iter()
            .map(|x| *x.nums.get(i).unwrap())
            .collect();
        numlists.push(numlist);
    }
    let mut newrules = HashSet::new();
    for rule in &day.rules {
        newrules.insert(rule.clone());
    }
    let rootstep = step(newrules, numlists);
    rootstep.unwrap()
}

fn day16b(input: &str) -> i64 {
    let answers = order_of_answers(input);
    let day = parse_input(input);
    let combos: Vec<(i64, String)> = day
        .my_ticket
        .nums
        .into_iter()
        .zip(answers)
        .filter(|(_, s)| s.contains("departure"))
        .collect();
    println!("{:?}", combos);
    combos.iter().map(|(a, _)| a).product()
}

pub fn day16(input: &str, part: char) -> i64 {
    match part {
        'a' => day16a(input),
        'b' => day16b(input),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day16;

    #[test]
    fn test_case() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .to_string();

        assert_eq!(day16::day16a(&input), 71);

        let input2 = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .to_string();

        //day16::day16b(&input);
        day16::day16b(&input2);
    }
}

