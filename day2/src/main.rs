use std::env;
use std::fs;
use std::ops::Range;
use utils;

struct PasswordRule {
    letter: char,
    count_range: Range<usize>,
    password: String
}

fn partition(str: &str, split_char: char) -> (&str, &str) {
    let delim_index: Option<usize> = str.find(split_char);

    match delim_index {
        Some(i) => {
            let (first, second) = str.split_at(i);
            (first, &second[1..])
        },
        None => ("", ""),
    }
}

fn rule_from_string(str: &str) -> PasswordRule {
    let (rule, password) = partition(str, ':');
    let (range, letter) = partition(&rule, ' ');
    let (lower, upper) = partition(&range, '-');
    let range_start = usize::from_str_radix(&lower, 10);
    let range_end = usize::from_str_radix(&upper, 10);
    let error_msg = format!("rule: {}, password: {}, range: {}, lower: {}, upper: {}", rule, password, range, lower, upper);

    PasswordRule {
        letter: String::from(letter).pop().unwrap(),
        count_range: Range {
            start: range_start.expect(error_msg.as_ref()),
            end: range_end.expect(error_msg.as_ref()) + 1
        },
        password: String::from(password.trim())
    }
}

fn password_valid(rule: &PasswordRule) -> bool {
    let incidences = rule.password.chars().filter(|c| *c == rule.letter).count();
    rule.count_range.contains(&incidences)
}

/* Part 2 */

struct PasswordRuleRevised {
    letter: char,
    position_1: usize,
    position_2: usize,
    password: String
}

fn revised_rule_from_string(str: &str) -> PasswordRuleRevised {
    let (rule, password) = partition(str, ':');
    let (range, letter) = partition(&rule, ' ');
    let (lower, upper) = partition(&range, '-');
    let first_pos = usize::from_str_radix(&lower, 10);
    let second_pos = usize::from_str_radix(&upper, 10);
    let error_msg = format!("rule: {}, password: {}, range: {}, lower: {}, upper: {}", rule, password, range, lower, upper);

    PasswordRuleRevised {
        letter: String::from(letter).pop().unwrap(),
        position_1: first_pos.expect(error_msg.as_ref()) - 1,
        position_2: second_pos.expect(error_msg.as_ref()) - 1,
        password: String::from(password.trim())
    }
}

fn password_valid_revised(rule: &PasswordRuleRevised) -> bool {
    (rule.password.chars().nth(rule.position_1) == Some(rule.letter)) ^ (rule.password.chars().nth(rule.position_2) == Some(rule.letter))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let rules: Vec<PasswordRule> = utils::split_lines(&contents).map(|s| rule_from_string(s)).collect();

    let valid_count = rules.iter().filter(|r| password_valid(r)).count();
    println!("{} valid passwords", valid_count);

    let revised_rules: Vec<PasswordRuleRevised> = utils::split_lines(&contents).map(|s| revised_rule_from_string(s)).collect();
    let revised_valid_count = revised_rules.iter().filter(|r| password_valid_revised(r)).count();
    println!("{} valid passwords using new rule", revised_valid_count);
}
