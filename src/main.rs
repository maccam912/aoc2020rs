mod util;
mod day1;
mod day2;
mod day3;

fn main() {
    // Day 1
    let nums = util::load_nums("inputs/day1.txt");
    let a = day1::day1(&nums, 2);
    let b = day1::day1(&nums, 3);
    println!("Day 1: A: {:?}, B: {:?}", a, b);
    
    // Day 2
    let lines = util::load_strings("inputs/day2.txt");
    let a = day2::day2(&lines, 'a');
    let b = day2::day2(&lines, 'b');
    println!("Day 2: A: {:?}, B: {:?}", a, b);

    // Day 3
    let lines = util::load_strings("inputs/day3.txt");
    let a = day3::day3(&lines, 'a');
    let b = day3::day3(&lines, 'b');
    println!("Day 3: A: {:?}, B: {:?}", a, b);
}

