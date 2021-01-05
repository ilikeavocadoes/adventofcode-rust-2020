use std::str::FromStr;

pub fn run() {
    let input = std::fs::read_to_string("input/day2.input").unwrap();
    let entries: Vec<PasswordEntry> = input.lines().map(|l| l.parse().unwrap()).collect();
    let number_valid = entries.iter().fold(0, |acc, e| {
        if follows_policy(&e.policy, &e.password) {
            acc + 1
        } else {
            acc
        }
    });
    let number_new_valid = entries.iter().fold(0, |acc, e| {
        if follows_new_company_policy(&e.policy, &e.password) {
            acc + 1
        } else {
            acc
        }
    });
    print!("{}\n", number_valid);
    print!("{}\n", number_new_valid);
}

#[derive(Debug)]
struct PasswordEntry {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Debug)]
struct PasswordPolicy {
    minimum_chars: usize,
    maximum_chars: usize,
    character: char,
}

impl FromStr for PasswordEntry {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let limits: Vec<&str> = parts[0].split('-').collect();
        let low = limits[0].parse().unwrap();
        let high = limits[1].parse().unwrap();
        let character = parts[1].chars().collect::<Vec<char>>()[0];
        let policy = PasswordPolicy {
            minimum_chars: low,
            maximum_chars: high,
            character: character,
        };
        let password = parts[2];
        Ok(PasswordEntry {
            policy: policy,
            password: String::from_str(password).unwrap(),
        })
    }
}

fn follows_policy(policy: &PasswordPolicy, password: &str) -> bool {
    let n = password
        .matches(policy.character)
        .collect::<Vec<&str>>()
        .len();
    policy.minimum_chars <= n && n <= policy.maximum_chars
}

fn follows_new_company_policy(policy: &PasswordPolicy, password: &str) -> bool {
    let correct_char_in_first =
        password.chars().nth(policy.minimum_chars - 1) == Some(policy.character);
    let correct_char_in_second =
        password.chars().nth(policy.maximum_chars - 1) == Some(policy.character);
    (correct_char_in_first && !correct_char_in_second)
        || (!correct_char_in_first && correct_char_in_second)
}
