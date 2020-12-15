mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod util;

fn main() {
    // Day 1
    let nums = util::load_nums("inputs/day01.txt");
    let a = day01::day01(&nums, 2);
    let b = day01::day01(&nums, 3);
    println!("Day 1: A: {:?}, B: {:?}", a, b);

    // Day 2
    let lines = util::load_strings("inputs/day02.txt");
    let a = day02::day02(&lines, 'a');
    let b = day02::day02(&lines, 'b');
    println!("Day 2: A: {:?}, B: {:?}", a, b);

    // Day 3
    let lines = util::load_strings("inputs/day03.txt");
    let a = day03::day03(&lines, 'a');
    let b = day03::day03(&lines, 'b');
    println!("Day 3: A: {:?}, B: {:?}", a, b);

    // Day 4
    let contents = util::load_contents("inputs/day04.txt");
    let a = day04::day04(contents.clone(), 'a');
    let b = day04::day04(contents, 'b');
    println!("Day 4: A: {:?}, B: {:?}", a, b);

    // Day 5
    let lines = util::load_strings("inputs/day05.txt");
    let a = day05::day05(&lines, 'a');
    let b = day05::day05(&lines, 'b');
    println!("Day 5: A: {:?}, B: {:?}", a, b);

    // Day 6
    let contents = util::load_contents("inputs/day06.txt");
    let a = day06::day06(&contents, 'a');
    let b = day06::day06(&contents, 'b');
    println!("Day 6: A: {:?}, B: {:?}", a, b);

    // Day 7
    let lines = util::load_strings("inputs/day07.txt");
    let a = day07::day07(&lines, 'a');
    let b = day07::day07(&lines, 'b');
    println!("Day 7: A: {:?}, B: {:?}", a, b);

    // Day 8
    let lines = util::load_strings("inputs/day08.txt");
    let a = day08::day08(&lines, 'a');
    let b = day08::day08(&lines, 'b');
    println!("Day 8: A: {:?}, B: {:?}", a, b);

    // Day 9
    let nums = util::load_nums("inputs/day09.txt");
    let a = day09::day09(&nums, 'a');
    let b = day09::day09(&nums, 'b');
    println!("Day 9: A: {:?}, B: {:?}", a, b);

    // Day 10
    let nums = util::load_nums("inputs/day10.txt");
    let a = day10::day10(&nums, 'a');
    let b = day10::day10(&nums, 'b');
    println!("Day 10: A: {:?}, B: {:?}", a, b);

    // Day 11
    let lines = util::load_strings("inputs/day11.txt");
    let a = day11::day11(&lines, 'a');
    let b = day11::day11(&lines, 'b');
    println!("Day 11: A: {:?}, B: {:?}", a, b);

    // Day 12
    let lines = util::load_strings("inputs/day12.txt");
    let a = day12::day12(&lines, 'a');
    let b = day12::day12(&lines, 'b');
    println!("Day 12: A: {:?}, B: {:?}", a, b);

    // Day 13
    let lines = util::load_strings("inputs/day13.txt");
    let a = day13::day13(&lines, 'a');
    let b = day13::day13(&lines, 'b');
    println!("Day 13: A: {:?}, B: {:?}", a, b);

    // Day 14
    let lines = util::load_strings("inputs/day14.txt");
    let a = day14::day14(&lines, 'a');
    let b = day14::day14(&lines, 'b');
    println!("Day 14: A: {:?}, B: {:?}", a, b);
}
