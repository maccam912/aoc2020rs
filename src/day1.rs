use itertools::Itertools;

fn sum(v: &[&i64]) -> i64 {
    let mut sum = 0;
    for _v in v {
        sum += *_v;
    }
    sum
}

fn prod(v: &[&i64]) -> i64 {
    let mut prod = 1;
    for _v in v {
        prod *= *_v;
    }
    prod
}

pub fn day1(numbers: &[i64], count: i64) -> i64 {
    let it = numbers.into_iter().combinations(count as usize);
    for set in it {
        if sum(&set) == 2020 {
            return prod(&set);
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn test_case() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(day1::day1(&nums, 2), 514579 as i64);
        assert_eq!(day1::day1(&nums, 3), 241861950 as i64);
    }
}