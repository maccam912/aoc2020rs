use std::collections::VecDeque;

fn is_valid(num: i64, v: &VecDeque<i64>) -> bool {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[i] != v[j] && v[i] + v[j] == num {
                return true;
            }
        }
    }
    false
}

fn prep(nums: &[i64], preamblelen: usize) -> VecDeque<i64> {
    let mut retval = VecDeque::new();
    for num in nums.iter().take(preamblelen) {
        retval.push_back(*num);
    }
    retval
}

fn find_contiguous_sum(nums: &[i64], target: i64) -> Vec<i64> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let sum: i64 = nums[i..j].iter().sum();
            if sum == target {
                return nums[i..j].to_vec();
            }
        }
    }
    vec![]
}

fn day9a(nums: &[i64], preamblelen: usize) -> i64 {
    let mut v = prep(nums, preamblelen);
    for num in nums.iter().skip(preamblelen) {
        if !is_valid(*num, &v) {
            return *num;
        } else {
            v.pop_front();
            v.push_back(*num);
        }
    }
    0
}

fn day9b(nums: &[i64], preamblelen: usize) -> i64 {
    let invalid_num = day9a(nums, preamblelen);
    let mut v = find_contiguous_sum(nums, invalid_num);
    v.sort_unstable();
    v[0] + v[v.len() - 1]
}

pub fn day9(nums: &[i64], part: char) -> i64 {
    match part {
        'a' => day9a(nums, 25),
        'b' => day9b(nums, 25),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use day9::find_contiguous_sum;

    use crate::day9;

    #[test]
    fn test_case() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let nums: Vec<i64> = input
            .split('\n')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        let v = day9::prep(&nums, 5);
        assert!(day9::is_valid(40, &v));
        assert_eq!(day9::day9a(&nums, 5), 127);

        assert_eq!(find_contiguous_sum(&nums, 127), vec![15, 25, 47, 40]);
    }
}
