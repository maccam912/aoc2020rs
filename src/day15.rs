use std::collections::HashMap;

pub fn day15(nums: &[i64], part: char) -> i64 {
    let mut turns: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut turn = 1;
    let mut seq: Vec<i64> = Vec::new();
    let partturn = match part {
        'a' => 2020,
        'b' => 30000000,
        _ => panic!(),
    };
    let maxturn = partturn + 2;
    for num in nums {
        turns.entry(*num).or_insert_with(Vec::new).push(turn);
        seq.push(*num);
        turn += 1;
    }
    loop {
        let lastnum: i64 = *seq.last().unwrap();
        let default: Vec<i64> = vec![0];
        let all_turns = turns.get(&lastnum).unwrap_or(&default);
        let most_recent_turn = *all_turns.last().unwrap();
        turns.entry(lastnum).or_insert_with(Vec::new).push(turn - 1);
        let diff = if most_recent_turn == 0 || most_recent_turn == turn - 1 {
            0
        } else {
            turn - most_recent_turn - 1
        };
        seq.push(diff);
        turn += 1;
        if turn > maxturn {
            break;
        }
    }
    *seq.get((partturn as usize) - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day15;

    #[test]
    fn test_case() {
        let input: Vec<i64> = "0
        3
        6"
        .split('\n')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

        assert_eq!(day15::day15(&input, 'a'), 436);
        assert_eq!(day15::day15(&input, 'b'), 175594);
    }
}
