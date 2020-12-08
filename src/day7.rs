use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug)]
struct Rule {
    color: String,
    contains: HashMap<String, i64>,
}

fn parse_rule(line: &str) -> Rule {
    let parts: Vec<&str> = line.trim().split(" bags contain ").collect();
    let parentcolor = parts[0];
    let contents = parts[1];
    let pieces: Vec<&str> = contents.split(',').collect();
    let mut contains: HashMap<String, i64> = HashMap::new();

    let re = Regex::new(r"^(?P<amount>\d+) (?P<color>[a-z ]*) bags?.?$").unwrap();
    for piece in pieces {
        for caps in re.captures_iter(piece.trim()) {
            contains.insert(
                caps["color"].to_string(),
                caps["amount"].parse::<i64>().unwrap(),
            );
        }
    }
    Rule {
        color: parentcolor.to_string(),
        contains,
    }
}

fn find_ancestors(rules: &[Rule], color: String) -> HashSet<String> {
    let mut parents: Vec<String> = Vec::new();
    for rule in rules {
        let has = rule.contains.get(&color);
        if has.is_some() {
            parents.push(rule.color.to_string());
        }
    }
    let mut allancestors: HashSet<String> = HashSet::new();
    for parent in parents {
        allancestors.insert(parent.clone());
        let ancestors = find_ancestors(rules, parent);
        for ancestor in ancestors {
            allancestors.insert(ancestor);
        }
    }
    allancestors
}

fn get_num_descendents(rules: &HashMap<String, HashMap<String, i64>>, color: &str) -> i64 {
    let rule = rules.get(color).unwrap();
    if rule.keys().len() == 0 {
        1
    } else {
        let mut sum = 0;
        for (k, v) in rule {
            let d = get_num_descendents(rules, k);
            //println!("{:?} contains {:?} {:?}, each containing {:?}.", color, v, k, d);
            sum += v * d;
        }
        sum + 1
    }
}

fn day7a(lines: &[String]) -> i64 {
    let rules: Vec<Rule> = lines.iter().map(|x| parse_rule(x)).collect();
    let allancestors = find_ancestors(&rules, "shiny gold".to_string());
    allancestors.len() as i64
}

fn day7b(lines: &[String]) -> i64 {
    let rules: Vec<Rule> = lines.iter().map(|x| parse_rule(x)).collect();
    let mut rulesmap = HashMap::new();
    for rule in rules {
        rulesmap.insert(rule.color, rule.contains);
    }
    get_num_descendents(&rulesmap, "shiny gold") - 1
}

pub fn day7(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day7a(lines),
        'b' => day7b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day7;

    #[test]
    fn test_case() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let lines: Vec<String> = input.split('\n').map(|x| x.to_string()).collect();
        assert_eq!(day7::day7a(&lines), 4);
        assert_eq!(day7::day7b(&lines), 32);

        let input2 = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";
        let lines: Vec<String> = input2.split('\n').map(|x| x.to_string()).collect();
        assert_eq!(day7::day7b(&lines), 126);
    }
}
