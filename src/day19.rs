use pest::{error::Error, iterators::Pairs, Parser};
use regex::Regex;

#[derive(Parser)]
#[grammar = "day19b.pest"]
pub struct Day19Parser;

pub fn parse_expr(s: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    Day19Parser::parse(Rule::answer, s)
}

fn fix_name_or_lit(s: &str) -> String {
    if s == "\"a\"" {
        "\"a\"".to_string()
    } else if s == "\"b\"" {
        "\"b\"".to_string()
    } else {
        return format!("_{}", s);
    }
}

fn and_str(s: &str) -> String {
    let parts = s.trim().split(' ');
    parts
        .map(|x| fix_name_or_lit(x))
        .collect::<Vec<String>>()
        .join(" ~ ")
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

fn split_ors(s: &str) -> String {
    let mut newstr = String::new();
    let re = Regex::new(r"^(?P<num>.+) = \{ (?P<a>[^|]+) \| (?P<b>[^|]+) \}$").unwrap();
    for line in s.split('\n') {
        if re.is_match(line) {
            println!("{}", line);
            let caps = re.captures(line).unwrap();
            newstr.push_str(&format!("{} = {{ {} }}", &caps["num"], &caps["a"]));
            newstr.push('\n');
            newstr.push_str(&format!("{} = {{ {} }}", &caps["num"], &caps["b"]));
            newstr.push('\n');
        } else {
            newstr.push_str(line);
            newstr.push('\n');
        }
    }
    newstr
}

pub fn day19(contents: &str) -> i64 {
    let mut parts = contents.split("\n\n");
    let rules = parts.next().unwrap();
    let lines = parts.next().unwrap();
    let pest_rules = split_ors(&parse_rules_to_pest(rules));
    println!("{}", pest_rules);

    let mut sum = 0;
    for line in lines.split('\n') {
        if parse_expr(line).is_ok() {
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
