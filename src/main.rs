use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Debug)]
struct Entry<'a> {
    letter: char,
    allowed_count: Range<usize>,
    password: &'a str,
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input)?;
    let entries = input.lines().map(parse_entry).collect::<Vec<Entry>>();
    let valid_passwords_count = entries
        .iter()
        .filter(valid_password_toboggan_policy)
        .count();
    println!("{:?}", valid_passwords_count);
    Ok(())
}

fn valid_password_sled_policy(entry: &&Entry) -> bool {
    let letter_count = entry.password.matches(entry.letter).count();
    entry.allowed_count.contains(&letter_count)
}

fn valid_password_toboggan_policy(entry: &&Entry) -> bool {
    let first_pos = entry.allowed_count.start - 1;
    let second_pos = entry.allowed_count.end - 2;
    let first_present = entry.password.chars().nth(first_pos) == Some(entry.letter);
    let second_present = entry.password.chars().nth(second_pos) == Some(entry.letter);
    (first_present && !second_present) || (!first_present && second_present)
}

fn parse_entry(line: &str) -> Entry {
    let split: Vec<&str> = line.split(":").collect();
    let rule = split[0];
    let mut rule_split = rule.split(" ");
    let range = rule_split.next().unwrap().trim();
    let character = rule_split.next().unwrap().trim();
    let mut range_split = range.split("-");
    let min: usize = range_split.next().unwrap().parse().unwrap();
    let max: usize = range_split.next().unwrap().parse().unwrap();
    Entry {
        letter: character.chars().nth(0).unwrap(),
        allowed_count: Range {
            start: min,
            end: max.checked_add(1).unwrap(),
        },
        password: split[1].trim(),
    }
}
