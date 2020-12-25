#![feature(deque_range)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use structopt::StructOpt;

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
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod infinite_field_3d;
mod infinite_field_4d;
mod util;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    day: i64,
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => {
            // Day 1
            let nums = util::load_nums("inputs/day01.txt");
            let a = day01::day01(&nums, 2);
            let b = day01::day01(&nums, 3);
            println!("Day 1: A: {:?}, B: {:?}", a, b);
        }
        2 => {
            // Day 2
            let lines = util::load_strings("inputs/day02.txt");
            let a = day02::day02(&lines, 'a');
            let b = day02::day02(&lines, 'b');
            println!("Day 2: A: {:?}, B: {:?}", a, b);
        }
        3 => {
            // Day 3
            let lines = util::load_strings("inputs/day03.txt");
            let a = day03::day03(&lines, 'a');
            let b = day03::day03(&lines, 'b');
            println!("Day 3: A: {:?}, B: {:?}", a, b);
        }
        4 => {
            // Day 4
            let contents = util::load_contents("inputs/day04.txt");
            let a = day04::day04(contents.clone(), 'a');
            let b = day04::day04(contents, 'b');
            println!("Day 4: A: {:?}, B: {:?}", a, b);
        }
        5 => {
            // Day 5
            let lines = util::load_strings("inputs/day05.txt");
            let a = day05::day05(&lines, 'a');
            let b = day05::day05(&lines, 'b');
            println!("Day 5: A: {:?}, B: {:?}", a, b);
        }
        6 => {
            // Day 6
            let contents = util::load_contents("inputs/day06.txt");
            let a = day06::day06(&contents, 'a');
            let b = day06::day06(&contents, 'b');
            println!("Day 6: A: {:?}, B: {:?}", a, b);
        }
        7 => {
            // Day 7
            let lines = util::load_strings("inputs/day07.txt");
            let a = day07::day07(&lines, 'a');
            let b = day07::day07(&lines, 'b');
            println!("Day 7: A: {:?}, B: {:?}", a, b);
        }
        8 => {
            // Day 8
            let lines = util::load_strings("inputs/day08.txt");
            let a = day08::day08(&lines, 'a');
            let b = day08::day08(&lines, 'b');
            println!("Day 8: A: {:?}, B: {:?}", a, b);
        }
        9 => {
            // Day 9
            let nums = util::load_nums("inputs/day09.txt");
            let a = day09::day09(&nums, 'a');
            let b = day09::day09(&nums, 'b');
            println!("Day 9: A: {:?}, B: {:?}", a, b);
        }
        10 => {
            // Day 10
            let nums = util::load_nums("inputs/day10.txt");
            let a = day10::day10(&nums, 'a');
            let b = day10::day10(&nums, 'b');
            println!("Day 10: A: {:?}, B: {:?}", a, b);
        }
        11 => {
            // Day 11
            let lines = util::load_strings("inputs/day11.txt");
            let a = day11::day11(&lines, 'a');
            let b = day11::day11(&lines, 'b');
            println!("Day 11: A: {:?}, B: {:?}", a, b);
        }
        12 => {
            // Day 12
            let lines = util::load_strings("inputs/day12.txt");
            let a = day12::day12(&lines, 'a');
            let b = day12::day12(&lines, 'b');
            println!("Day 12: A: {:?}, B: {:?}", a, b);
        }
        13 => {
            // Day 13
            let lines = util::load_strings("inputs/day13.txt");
            let a = day13::day13(&lines, 'a');
            let b = day13::day13(&lines, 'b');
            println!("Day 13: A: {:?}, B: {:?}", a, b);
        }
        14 => {
            // Day 14
            let lines = util::load_strings("inputs/day14.txt");
            let a = day14::day14(&lines, 'a');
            let b = day14::day14(&lines, 'b');
            println!("Day 14: A: {:?}, B: {:?}", a, b);
        }
        15 => {
            // Day 15
            let lines = util::load_nums("inputs/day15.txt");
            let a = day15::day15(&lines, 'a');
            let b = day15::day15(&lines, 'b');
            println!("Day 15: A: {:?}, B: {:?}", a, b);
        }
        16 => {
            // Day 16
            let contents = util::load_contents("inputs/day16.txt");
            let a = day16::day16(&contents, 'a');
            let b = day16::day16(&contents, 'b');
            println!("Day 16: A: {:?}, B: {:?}", a, b);
        }
        17 => {
            // Day 17
            let contents = util::load_contents("inputs/day17.txt");
            let a = day17::day17(&contents, 'a');
            let b = day17::day17(&contents, 'b');
            println!("Day 17: A: {:?}, B: {:?}", a, b);
        }
        18 => {
            // Day 18
            let lines = util::load_strings("inputs/day18.txt");
            let a = day18::day18(&lines, 'a');
            let b = day18::day18(&lines, 'b');
            println!("Day 18: A: {:?}, B: {:?}", a, b);
        }
        19 => {
            // Day 19
            let contents = util::load_contents("inputs/day19.txt");
            let a = day19::day19(&contents, 'a');
            let b = day19::day19(&contents, 'b');
            println!("Day 19: A: {:?}, B: {:?}", a, b);
        }
        20 => {
            // Day 20
            let contents = util::load_contents("inputs/day20.txt");
            let a = day20::day20(&contents, 'a');
            let b = day20::day20(&contents, 'b');
            println!("Day 20: A: {:?}, B: {:?}", a, b);
        }
        21 => {
            //use std::fs::File;
            //use std::io::prelude::*;

            // Day 21
            let contents = util::load_contents("inputs/day21.txt");
            //let a = day21::gen_minizinc_prog(&contents);
            //let mut file = File::create("day21.mzn").unwrap();
            //file.write_all(a.as_bytes()).unwrap();
            let a = day21::day21a(&contents);
            println!("Day 21: A: {:?}", a);
        }
        22 => {
            // Day 22
            let contents = util::load_contents("inputs/day22.txt");
            let a = day22::day22(&contents, 'a');
            let b = day22::day22(&contents, 'b');
            println!("Day 22: A: {:?}, B: {:?}", a, b);
        }
        23 => {
            // Day 23
            let a = day23::day23a("643719258");
            let b = day23::day23b("643719258");
            println!("Day 23: A: {:?}, B: {:?}", a, b);
        }
        24 => {
            // Day 24
            let lines = util::load_strings("inputs/day24.txt");
            let a = day24::day24a(&lines);
            println!("Day 24: A: {:?}", a);
        }
        _ => panic!(format!(
            "Cannot run day {:?}, it doesn't exist maybe?",
            opt.day
        )),
    }
}
