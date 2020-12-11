use std::collections::{HashMap, HashSet};

use regex::Regex;

fn split_groups(contents: &str) -> Vec<String> {
    let separator = Regex::new(r"(\n *\n|\r\n *\r\n)").expect("Invalid regex");
    contents.split(&separator).map(|x| x.to_string()).collect()
}

fn make_set_union(answers: &str) -> HashSet<char> {
    let mut answerset: HashSet<char> = HashSet::new();
    for c in answers.chars() {
        if ('a'..='z').contains(&c) {
            answerset.insert(c);
        }
    }
    answerset
}

fn make_set_intersection(answers: &str) -> HashSet<char> {
    let people = answers.split('\n');
    let numpeople = people.clone().count();
    let mut answer: HashMap<char, i64> = HashMap::new();
    for person in people {
        for c in person.chars() {
            let prev_val: i64 = *answer.get(&c).or(Some(&0)).unwrap();
            answer.insert(c, prev_val + 1);
        }
    }

    let mut answerset = HashSet::new();
    for (k, v) in answer {
        if v == numpeople as i64 {
            answerset.insert(k);
        }
    }

    answerset
}

fn day06a(contents: &str) -> i64 {
    let groups = split_groups(contents);
    let answersets = groups.into_iter().map(|g| make_set_union(&g));
    let counts = answersets.map(|set| set.len());
    let sum: usize = counts.sum();
    sum as i64
}

fn day06b(contents: &str) -> i64 {
    let groups = split_groups(contents);
    let answersets = groups.into_iter().map(|g| make_set_intersection(&g));
    let counts = answersets.map(|set| set.len());
    let sum: usize = counts.sum();
    sum as i64
}

pub fn day06(contents: &str, part: char) -> i64 {
    match part {
        'a' => day06a(contents),
        'b' => day06b(contents),
        _ => 0,
    }
}
#[cfg(test)]
mod tests {
    use crate::day06;
    use std::collections::HashSet;

    #[test]
    fn test_case() {
        let input = "abcx
        abcy
        abcz";
        let mut abcxyz: HashSet<char> = HashSet::new();
        for c in vec!['a', 'b', 'c', 'x', 'y', 'z'] {
            abcxyz.insert(c);
        }

        assert_eq!(abcxyz, day06::make_set_union(input));

        let newinput = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";

        let ansa = day06::day06a(newinput);
        assert_eq!(ansa, 11);
        let ansb = day06::day06b(newinput);
        assert_eq!(ansb, 6);
    }
}
