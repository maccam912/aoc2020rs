struct Rule {
    lo: i64,
    hi: i64,
    letter: char,
}

struct Line {
    rule: Rule,
    password: String,
}

fn parse_rule(s: &str) -> Rule {
    let parts: Vec<&str> = s.split(' ').collect();
    let range: Vec<&str> = parts[0].split('-').collect();
    Rule {
        lo: range[0].parse::<i64>().unwrap(),
        hi: range[1].parse::<i64>().unwrap(),
        letter: parts[1].chars().next().unwrap(),
    }
}

fn parse_line(s: &str) -> Line {
    let parts: Vec<&str> = s.trim().split(": ").collect();
    let rule = parse_rule(parts[0]);
    Line {
        rule,
        password: parts[1].to_string(),
    }
}

fn validate_a(line: &Line) -> bool {
    let filtered: Vec<char> = line
        .password
        .chars()
        .filter(|c| c == &line.rule.letter)
        .collect();
    filtered.len() >= line.rule.lo as usize && filtered.len() <= line.rule.hi as usize
}

fn validate_b(line: &Line) -> bool {
    let chararray: Vec<char> = line.password.chars().collect();
    let p1 = chararray.get((line.rule.lo - 1) as usize);
    let p2 = chararray.get((line.rule.hi - 1) as usize);
    (p1.is_some() && p2.is_some())
        && ((p1.unwrap() == &line.rule.letter) ^ (p2.unwrap() == &line.rule.letter))
}

fn validate(line: &Line, part: char) -> bool {
    match part {
        'a' => validate_a(line),
        'b' => validate_b(line),
        _ => false,
    }
}

pub fn day02(lines: &[String], part: char) -> i64 {
    let valid_lines: Vec<Line> = lines
        .iter()
        .map(|line| parse_line(line))
        .filter(|line: &Line| validate(line, part))
        .collect();
    valid_lines.len() as i64
}

#[cfg(test)]
mod tests {
    use crate::day02;

    #[test]
    fn test_case() {
        let text: Vec<String> = "1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc"
            .split("\n")
            .map(|s| s.to_string())
            .collect();
        let ansa = day02::day02(&text, 'a');
        assert_eq!(2, ansa);
        let ansb = day02::day02(&text, 'b');
        assert_eq!(1, ansb);
    }
}
