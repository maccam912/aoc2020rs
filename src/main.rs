mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

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

    // Day 4
    let contents = util::load_contents("inputs/day4.txt");
    let a = day4::day4(contents.clone(), 'a');
    let b = day4::day4(contents, 'b');
    println!("Day 4: A: {:?}, B: {:?}", a, b);

    // Day 5
    let lines = util::load_strings("inputs/day5.txt");
    let a = day5::day5(&lines, 'a');
    let b = day5::day5(&lines, 'b');
    println!("Day 5: A: {:?}, B: {:?}", a, b);

    // Day 6
    let contents = util::load_contents("inputs/day6.txt");
    let a = day6::day6(&contents, 'a');
    let b = day6::day6(&contents, 'b');
    println!("Day 6: A: {:?}, B: {:?}", a, b);
}
