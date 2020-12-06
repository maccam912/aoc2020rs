use std::fs;

pub fn load_contents(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn load_strings(path: &str) -> Vec<String> {
    let contents = load_contents(path);
    let lines = contents.split('\n');
    lines.map(|x| x.trim().to_string()).collect()
}

pub fn load_nums(path: &str) -> Vec<i64> {
    let lines = load_strings(path);
    let ints = lines
        .into_iter()
        .map(|x| x.trim().parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap());
    ints.collect()
}
