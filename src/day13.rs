fn parse_lines(lines: &[String]) -> (i64, Vec<Option<i64>>) {
    let eta: i64 = lines[0].parse::<i64>().unwrap();
    let buslist: Vec<Option<i64>> = lines[1]
        .split(',')
        .map(|x| {
            if x.parse::<i64>().is_ok() {
                Some(x.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .collect();
    (eta, buslist)
}

fn day13a(lines: &[String]) -> i64 {
    let (eta, buslist) = parse_lines(lines);
    let mut waittimes: Vec<(i64, i64)> = buslist
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|x| (x, x - (eta % x)))
        .collect();
    waittimes.sort_by(|a, b| a.1.cmp(&b.1));
    let (busid, wait) = waittimes[0];
    busid * wait
}

#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn day13b(lines: &[String]) -> i64 {
    let (_, buslist) = parse_lines(lines);
    let mut residues: Vec<i64> = Vec::new();
    let mut modulii: Vec<i64> = Vec::new();
    for (i, opt_bus) in buslist.iter().enumerate() {
        if let Some(bus) = opt_bus {
            residues.push(bus - i as i64);
            modulii.push(*bus);
        }
    }
    let c = chinese_remainder(&residues, &modulii);
    c.unwrap()
}

pub fn day13(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day13a(lines),
        'b' => day13b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day13;

    #[test]
    fn test_case() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let lines: Vec<String> = input.split('\n').map(|x| x.trim().to_string()).collect();
        assert_eq!(day13::day13a(&lines), 295);
        let l2: Vec<String> = "939
        17,x,13,19"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();
        assert_eq!(day13::day13b(&l2), 3417);
        let l3: Vec<String> = "939
        67,7,59,61"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();
        assert_eq!(day13::day13b(&l3), 754018);
        assert_eq!(day13::day13b(&lines), 1068781);
    }
}
