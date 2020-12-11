use std::collections::HashMap;

use regex::Regex;

#[allow(dead_code)]
struct Document {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

fn split_documents(s: &str) -> Vec<String> {
    let separator = Regex::new(r"(\n *\n|\r\n *\r\n)").expect("Invalid regex");
    s.split(&separator).map(|x| x.to_string()).collect()
}

fn parse_doc(doc: &str) -> Option<Document> {
    let mut fields: HashMap<&str, String> = HashMap::new();
    doc.split_whitespace()
        .filter(|field| field.contains(':'))
        .for_each(|field| {
            let parts: Vec<&str> = field.split(':').collect();
            fields.insert(parts[0], parts[1].to_string());
        });
    let _cid = fields.get("cid");
    let cid = match _cid {
        Some(cid) => Some(cid.clone()),
        None => None,
    };

    let doc = Document {
        byr: fields.get("byr")?.clone(),
        iyr: fields.get("iyr")?.clone(),
        eyr: fields.get("eyr")?.clone(),
        hgt: fields.get("hgt")?.clone(),
        hcl: fields.get("hcl")?.clone(),
        ecl: fields.get("ecl")?.clone(),
        pid: fields.get("pid")?.clone(),
        cid,
    };
    Some(doc)
}

fn validate_years(doc: &Document) -> bool {
    (doc.byr.parse::<i64>().is_ok()
        && doc.byr.parse::<i64>().unwrap() >= 1920
        && doc.byr.parse::<i64>().unwrap() <= 2002)
        && (doc.iyr.parse::<i64>().is_ok()
            && doc.iyr.parse::<i64>().unwrap() >= 2010
            && doc.iyr.parse::<i64>().unwrap() <= 2020)
        && (doc.eyr.parse::<i64>().is_ok()
            && doc.eyr.parse::<i64>().unwrap() >= 2020
            && doc.eyr.parse::<i64>().unwrap() <= 2030)
}

fn validate_hgt(doc: &Document) -> bool {
    let l = doc.hgt.len();
    let unit = doc.hgt.get(l - 2..).unwrap();
    let _num = doc.hgt.get(..l - 2).unwrap().parse::<i64>();
    if _num.is_err() {
        return false;
    }
    let num = _num.unwrap();
    match unit {
        "cm" => (150..=193).contains(&num),
        "in" => (59..=76).contains(&num),
        _ => false,
    }
}

fn validate_hcl(doc: &Document) -> bool {
    let r = Regex::new(r"#[0-9a-f]{6}").unwrap();
    doc.hcl.matches(&r).count() > 0
}

fn validate_ecl(doc: &Document) -> bool {
    let ecl: &str = &doc.ecl;
    matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn validate_pid(doc: &Document) -> bool {
    let r = Regex::new(r"^\d{9}$").unwrap();
    r.is_match(&doc.pid)
}

fn validate(doc: &Document) -> bool {
    validate_years(doc)
        && validate_hgt(doc)
        && validate_hcl(doc)
        && validate_ecl(doc)
        && validate_pid(doc)
}

fn day04a(contents: String) -> usize {
    let docstrings = split_documents(&contents);
    let docs: Vec<Option<Document>> = docstrings.into_iter().map(|doc| parse_doc(&doc)).collect();
    docs.into_iter().filter(|doc| doc.is_some()).count()
}

fn day04b(contents: String) -> usize {
    let docstrings = split_documents(&contents);
    let docs: Vec<Option<Document>> = docstrings.into_iter().map(|doc| parse_doc(&doc)).collect();
    docs.into_iter()
        .filter(|doc| doc.is_some() && validate(doc.as_ref().unwrap()))
        .count()
}

pub fn day04(contents: String, part: char) -> usize {
    match part {
        'a' => day04a(contents),
        'b' => day04b(contents),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day04;

    #[test]
    fn test_case() {
        let test_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let docstrings = day04::split_documents(&test_input);
        assert_eq!(docstrings.len(), 4);

        let docs: Vec<Option<day04::Document>> = docstrings
            .into_iter()
            .map(|doc| day04::parse_doc(&doc))
            .collect();
        let valid_docs_num = docs.into_iter().filter(|doc| doc.is_some()).count();
        assert_eq!(valid_docs_num, 2);
    }
}
