#![allow(clippy::many_single_char_names)]

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum PType {
    Terminal(String),
    NonTerminal((String, String)),
}

#[derive(Debug)]
struct ProductionRule {
    name: String,
    produces: PType,
}

fn parse_rule_line(s: &str) -> Vec<ProductionRule> {
    let mut rules = Vec::new();
    let mut parts = s.split(": ");
    let rulename = parts.next().unwrap();
    let produces = parts.next().unwrap();
    for section in produces.split('|') {
        if section == "\"a\"" {
            let rule = ProductionRule {
                name: rulename.to_string(),
                produces: PType::Terminal("a".to_string()),
            };
            rules.push(rule);
        } else if section == "\"b\"" {
            let rule = ProductionRule {
                name: rulename.to_string(),
                produces: PType::Terminal("b".to_string()),
            };
            rules.push(rule);
        } else {
            let mut pieces = section.trim().split(' ');
            //println!("{}", section);
            let len_pieces = pieces.clone().count();
            assert_eq!(len_pieces, 2);
            let ptype = PType::NonTerminal((
                pieces.next().unwrap().to_string(),
                pieces.next().unwrap().to_string(),
            ));
            let rule = ProductionRule {
                name: rulename.to_string(),
                produces: ptype,
            };
            rules.push(rule);
        }
    }
    rules
}

fn parse_rules(s: &str) -> Vec<ProductionRule> {
    let mut rulesmap = Vec::new();
    for line in s.split('\n') {
        let rules = parse_rule_line(line);
        for rule in rules {
            rulesmap.push(rule);
        }
    }
    rulesmap
}

fn check_string_against_rules(input: &str, rules: &[ProductionRule]) -> bool {
    let mut p: HashMap<(usize, usize, usize), bool> = HashMap::new();

    // Initialize p
    for a in 1..=input.len() {
        for b in 1..=input.len() {
            for rule in rules {
                p.insert((a, b, rule.name.parse::<usize>().unwrap()), false);
            }
        }
    }

    // First round
    for (i, c) in input.chars().enumerate() {
        let cstr = format!("{}", c);
        for rule in rules {
            if rule.produces == PType::Terminal(cstr.clone()) {
                //println!("First round: ({:?}, {:?}, {:?}) = true", 1, i+1, rule.name.parse::<usize>().unwrap());
                p.insert((1, i + 1, rule.name.parse::<usize>().unwrap()), true);
            }
        }
    }

    let n = input.len();
    for l in 2..=n {
        for s in 1..=n - l + 1 {
            for _p in 1..=l - 1 {
                for rule in rules {
                    let a = rule.name.parse::<usize>().unwrap();
                    if let PType::NonTerminal((bb, cc)) = &rule.produces {
                        let b = bb.parse::<usize>().unwrap();
                        let c = cc.parse::<usize>().unwrap();
                        // if P[p,s,b] and P[l-p,s+p,c] then set P[l,s,a] = true
                        if *p.get(&(_p, s, b)).unwrap() && *p.get(&(l - _p, s + _p, c)).unwrap() {
                            //println!("Both ({:?},{:?},{:?}) and ({:?},{:?},{:?}) are true!. Setting ({:?},{:?},{:?}) to true.", _p,s,b,l-_p,s+_p,c,l,s,a);
                            p.insert((l, s, a), true);
                        }
                    }
                }
            }
        }
    }

    //println!("Finally, checking ({:?},1,0). {:?}", input.len(), p.get(&(input.len(), 1, 0)));
    *p.get(&(input.len(), 1, 0)).unwrap()
}

pub fn day19a(contents: &str) -> i64 {
    let mut parts = contents.split("\n\n");
    let rules = parts.next().unwrap();
    let inputs = parts.next().unwrap();
    let rulesmap = parse_rules(rules);
    let mut valid_strings = Vec::new();
    for line in inputs.split('\n') {
        if check_string_against_rules(line, &rulesmap) {
            valid_strings.push(line);
        }
    }
    valid_strings.len() as i64
}

pub fn day19b(contents: &str) -> i64 {
    let mut parts = contents.split("\n\n");
    let rules = parts.next().unwrap();
    let inputs = parts.next().unwrap();
    let mut newrules = rules.to_string();
    newrules.push_str("\n8: 42 8");
    newrules.push_str("\n11: 42 150");
    newrules.push_str("\n150: 11 31");
    let rulesmap = parse_rules(&newrules);
    let mut valid_strings = Vec::new();
    for line in inputs.split('\n') {
        if check_string_against_rules(line, &rulesmap) {
            valid_strings.push(line);
        }
    }
    valid_strings.len() as i64
}

pub fn day19(contents: &str, part: char) -> i64 {
    match part {
        'a' => day19a(contents),
        'b' => day19b(contents),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day19;
    use crate::util;

    #[test]
    fn test_case() {
        let contents = util::load_contents("inputs/day19test.test");
        let mut parts = contents.split("\n\n");
        let rules = parts.next().unwrap();
        let day19a = day19::day19(&contents, 'a');
        assert_eq!(day19a, 3);
        let day19b = day19::day19(&contents, 'b');
        assert_eq!(day19b, 12);
    }
}
