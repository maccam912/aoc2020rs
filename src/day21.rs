#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::util;

fn parse_allergens(s: &str) -> Vec<String> {
    let re = Regex::new(r"\(contains (?P<allergens_list>.*)\)$").unwrap();
    println!("{:?}", s);
    let caps = re.captures(s).unwrap();
    caps["allergens_list"]
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn parse_ingredients(s: &str) -> Vec<String> {
    let re = Regex::new(r"^(?P<ingredients_list>.*) \(contains .*\)$").unwrap();
    println!("{:?}", s);
    let caps = re.captures(s).unwrap();
    caps["ingredients_list"]
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn all_allergens(contents: &str) -> HashSet<String> {
    let lines = contents.split('\n');
    let mut allergens_set = HashSet::new();
    for line in lines {
        let allergens = parse_allergens(line);
        for a in allergens {
            allergens_set.insert(a);
        }
    }
    // We have 8 allergens: ["dairy", "eggs", "fish", "nuts", "peanuts", "sesame", "soy", "wheat"]
    allergens_set
}

fn all_ingredients(contents: &str) -> HashSet<String> {
    let lines = contents.split('\n');
    let mut ingredients_set = HashSet::new();
    for line in lines {
        let ingredients = parse_ingredients(line);
        for a in ingredients {
            ingredients_set.insert(a);
        }
    }
    ingredients_set
}

fn get_line_constraints(s: &str) -> String {
    let re = Regex::new(r"^(?P<ingredients_list>.*) \(contains (?P<allergens_list>.*)\)$").unwrap();
    let mut constraints = Vec::new();
    let mut indices = HashMap::new();
    for (i, ingredient) in [
        "dairy", "eggs", "fish", "nuts", "peanuts", "sesame", "soy", "wheat",
    ]
    .iter()
    .enumerate()
    {
        //for (i, ingredient) in ["dairy", "fish", "soy"].iter().enumerate() {
        indices.insert(*ingredient, i);
    }
    let caps = re.captures(s).unwrap();
    for allergen in caps["allergens_list"].split(", ") {
        let idx = indices.get(allergen).unwrap();
        let ingredients_strings: Vec<String> = caps["ingredients_list"]
            .split(' ')
            .map(|x| format!("{}[{:?}]", x, idx + 1))
            .collect();
        constraints.push(format!(
            "constraint {} == 1;",
            ingredients_strings.join(" + ")
        ));
    }
    constraints.join("\n")
}

pub fn gen_minizinc_prog(s: &str) -> String {
    let mut prog_lines = Vec::new();

    // Create variables (eac ingredient is a bool array)
    let ingredients = all_ingredients(s);
    for ingredient in &ingredients {
        let s = format!("array[1..8] of var bool: {};", ingredient);
        //let s = format!("array[1..3] of var bool: {};", ingredient);
        prog_lines.push(s);
        let s2 = format!("constraint sum({}) <= 1;", ingredient);
        prog_lines.push(s2);
    }
    prog_lines.push("\n".to_string());

    for i in 1..=8 {
        //for i in 1..=3 {
        let ingredients_list: Vec<String> = (&ingredients)
            .iter()
            .map(|x| format!("{}[{}]", x, i))
            .collect();
        prog_lines.push(format!("constraint {} == 1;", ingredients_list.join("+")));
    }
    for line in s.split('\n') {
        let constraints = get_line_constraints(line);
        prog_lines.push(constraints);
    }

    prog_lines.join("\n")
}

pub fn day21a(s: &str) -> i64 {
    let ingredients: Vec<String> = util::load_strings("inputs/day21results.txt")
        .iter()
        .map(|x| x.trim().to_string())
        .collect();
    let lines = s.split('\n');
    let mut sum = 0;
    for line in lines {
        for ingredient in &parse_ingredients(line) {
            if ingredients.contains(ingredient) {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use day21::all_allergens;

    use crate::{day21, util};

    #[test]
    fn test_case() {
        assert_eq!(
            day21::parse_allergens("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            vec!["dairy", "fish"]
        );
        let test_contents = util::load_contents("inputs/day21test.test");
        let all = all_allergens(&test_contents);
        let mut test_allergens: HashSet<String> = HashSet::new();
        for a in ["fish", "dairy", "soy"].iter() {
            test_allergens.insert(a.to_string());
        }
        assert_eq!(all, test_allergens);
    }
}
