use std::collections::HashMap;

use nom::{IResult, bytes::complete::tag, sequence::{pair, preceded, tuple}};
use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "day19test.pest"]
pub struct Day19Parser;

pub fn parse_expr(s: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    Day19Parser::parse(Rule::answer, s)
}

fn fix_name_or_lit(s: &str) -> String {
    if s == "\"a\"" {
        return "\"a\"".to_string();
    } else if s == "\"b\"" {
        return "\"b\"".to_string();
    } else {
        return format!("_{}", s);
    }
}

fn and_str(s: &str) -> String {
    let parts = s.trim().split(' ');
    parts.map(|x| fix_name_or_lit(x)).collect::<Vec<String>>().join(" ~ ")
}

fn parse_rule(s: &str) -> String {
    let ors = s.split('|');
    let mut retval = "{ ".to_string();
    let orparts: Vec<String> = ors.map(|s| and_str(s)).collect();
    retval.push_str(&orparts.join(" | "));
    retval.push_str(" }");
    retval
}

fn parse_rules_to_pest(input: &str) -> String {
    let lines = input.split('\n').map(|x| x.trim());
    let mut newlines = vec!["answer = { SOI ~ _0 ~ EOI }".to_string()];
    for line in lines {
        let mut parts = line.split(':');
        let rulenum = parts.next().unwrap();
        let rulenumident = format!("_{}", rulenum);
        let rule = parts.next().unwrap();
        let rulestr = parse_rule(rule);
        newlines.push(format!("{} = {}", rulenumident, rulestr));
    }
    newlines.join("\n")
}

fn parse_a(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}
fn parse_b(input: &str) -> IResult<&str, &str> {
    tag("b")(input)
}

type ParserFn<'a> = &'a dyn FnMut(&str) -> IResult<&str, &str>;

fn all_rules_defined(rule: &str, rulemap: &HashMap<i64, ParserFn>) -> bool {
    true
}

fn parse_and_rule(rule: &str, rulenum: i64, rulemap: &mut HashMap<i64, ParserFn>) {
    let rules: Vec<&ParserFn> = rule.split(' ').map(|x| x.parse::<i64>().unwrap()).map(|x| rulemap.get(&x).unwrap()).collect();
    let mut prevrule: ParserFn = rules[0];
    for i in 1..rules.len() {
        prevrule = &preceded(prevrule, rules[i]);
    }
    rulemap.insert(rulenum, prevrule);
}

fn parse_or_rule(rule: &str, rulemap: &mut HashMap<i64, ParserFn>) {
    //and_rules
}

fn parse_rules_to_nom(lines: &str, rulemap: &mut HashMap<i64, ParserFn>) {
    for line in lines.split('\n') {
        let mut parts = line.split(':');
        let rulenum = parts.next().unwrap().parse::<i64>().unwrap();
        let rule = parts.next().unwrap().trim();
        if rule == "\"a\"" {
            rulemap.insert(rulenum, &parse_a);
        } else if rule == "\"b\"" {
            rulemap.insert(rulenum, &parse_b);
        } else {
            if all_rules_defined(rule, &rulemap) {
                if rule.contains('|') {
                    //parse_or_rule(rule);
                } else {
                    parse_and_rule(rule, rulenum, rulemap);
                }
            }
        }
    }
}

pub fn day19(contents: &str) -> i64 {
    let mut parts = contents.split("\n\n");
    let rules = parts.next().unwrap();
    let lines = parts.next().unwrap();
    let mut rulemap: HashMap<i64, ParserFn> = HashMap::new();
    parse_rules_to_nom(rules, &mut rulemap);
    let mut sum = 0;
    for line in lines.split('\n') {
        if rulemap.get(&14).unwrap()(line).is_ok() {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day19;

    #[test]
    fn test_case() {
        let input1 = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";
        let regex = day19::parse_rules_to_pest(input1);
        println!("{:?}", regex);

        let input2 = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"";
        let regex = day19::parse_rules_to_pest(input2);
        println!("{:?}", regex);

        let input3 = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        //assert_eq!(day19::day19(input3, 'a'), 2);
    }
}