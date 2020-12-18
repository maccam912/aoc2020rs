use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::PrecClimber,
    Parser,
};

#[derive(Parser)]
#[grammar = "day18.pest"]
pub struct Day18Parser;

pub fn parse_expr(s: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    Day18Parser::parse(Rule::expr, s)
}

pub fn calculate(pairs: Pairs<Rule>) -> i64 {
    let mut operands = Vec::new();
    let mut operator = '.';
    for pair in pairs {
        // First check if we can reduce what we have so far
        if operands.len() == 2 && "+*".contains(operator) {
            // we have an operator and two operands, lets compute...
            let tmp = match operator {
                '+' => operands[0] + operands[1],
                '*' => operands[0] * operands[1],
                _ => panic!(),
            };
            operator = '.';
            operands = vec![tmp];
        }
        match pair.as_rule() {
            Rule::num => operands.push(pair.as_span().as_str().parse::<i64>().unwrap()),
            Rule::add => operator = '+',
            Rule::multiply => operator = '*',
            Rule::expr => operands.push(calculate(pair.into_inner())),
            _ => panic!("{:?}", pair.as_rule()),
        }
    }
    match operands.len() {
        1 => operands[0],
        2 => match operator {
            '+' => operands[0] + operands[1],
            '*' => operands[0] * operands[1],
            _ => panic!("Operator was %{:?}", operator),
        },
        _ => panic!("More than 2 operands"),
    }
}

fn day18a(s: &[String]) -> i64 {
    let mut sum = 0;
    for line in s {
        let pairs: Result<Pairs<Rule>, Error<Rule>> = parse_expr(line);
        let ans = calculate(pairs.unwrap());
        sum += ans;
    }
    sum
}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::{Assoc::*, Operator};
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left),
            Operator::new(add, Left),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn day18b(s: &[String]) -> i64 {
    let mut sum = 0;
    for line in s {
        let pairs: Result<Pairs<Rule>, Error<Rule>> = parse_expr(line);
        let ans = eval(pairs.unwrap());
        sum += ans;
    }
    sum
}

pub fn day18(s: &[String], part: char) -> i64 {
    match part {
        'a' => day18a(s),
        'b' => day18b(s),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day18;

    #[test]
    fn test_case() {
        let parse1 = day18::parse_expr("1 + 2 * 3 + 4 * 5 + 6");
        assert!(parse1.is_ok());
        let parse2 = day18::parse_expr("1 + (2 * 3) + (4 * (5 + 6))");
        assert!(parse2.is_ok());

        let answer1 = day18::day18a(&["1 + 2".to_string()]);
        assert_eq!(answer1, 3);
        let answer2 = day18::day18a(&["1 + 2 * 3 + 4 * 5 + 6".to_string()]);
        assert_eq!(answer2, 71);
        let answer3 = day18::day18a(&["1 + (2 * 3) + (4 * (5 + 6))".to_string()]);
        assert_eq!(answer3, 51);
        let answer4 =
            day18::day18a(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()]);
        assert_eq!(answer4, 13632);
        let answer5 = day18::day18a(&[
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),
        ]);
        assert_eq!(answer5, 13632 + 12240);

        let answer1 = day18::day18b(&["1 + 2".to_string()]);
        assert_eq!(answer1, 3);
        let answer2 = day18::day18b(&["1 + 2 * 3 + 4 * 5 + 6".to_string()]);
        assert_eq!(answer2, 231);
        let answer3 = day18::day18b(&["1 + (2 * 3) + (4 * (5 + 6))".to_string()]);
        assert_eq!(answer3, 51);
        let answer4 =
            day18::day18b(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()]);
        assert_eq!(answer4, 23340);
        let answer5 = day18::day18b(&[
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),
        ]);
        assert_eq!(answer5, 692400);
    }
}
