use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rules, messages) = parse(&input);

    println!("part one: {:?}", part_one(&rules, &messages));
    println!("part two: {:?}", part_two(rules, &messages));
}

fn part_one(rules: &Rules, messages: &[Message]) -> usize {
    messages.iter().filter(|m| is_valid(rules, &m)).count()
}

fn part_two(mut rules: Rules, messages: &[Message]) -> usize {
    // 0 : 8 11
    // 8 : 42 | 42 8
    // 11 : 42 31 | 42 11 31
    // is equivalent to
    // 0 : 42... (n times) 42 (m times)... 31 (m times)...
    // with n, m in [1, 5] is enough
    let mut r0 = Vec::new();
    let n = 5;
    let m = 5;
    for r11 in 1..=m {
        for r8 in 1..=n {
            let mut sr0 = vec![42; r8];
            sr0.append(&mut vec![42; r11]);
            sr0.append(&mut vec![31; r11]);

            r0.push(sr0);
        }
    }
    r0.sort_by_key(|v| v.len());
    r0.reverse(); // rules are processed in order and to be greedy it is sorted by descending length
    *rules.get_mut(&0).unwrap() = Rule::Composition(r0);
    rules.remove(&8);
    rules.remove(&11);

    messages.iter().filter(|m| is_valid(&rules, &m)).count()
}

type Message = Vec<u8>;

type Messages = Vec<Message>;

type Rules = HashMap<u64, Rule>;

fn parse(s: &str) -> (Rules, Messages) {
    let mut it = s.split("\n\n");
    let rules = it
        .next()
        .expect("Cannot read rules")
        .lines()
        .map(|l| {
            let mut it = l.split(':');
            let rule_number = it
                .next()
                .expect("Cannot parse rule number")
                .parse()
                .expect("Cannot parse rule number as integer");
            let rule = it
                .next()
                .expect("Cannot parse text rule")
                .trim()
                .parse()
                .expect("Cannot parse rule");
            (rule_number, rule)
        })
        .collect();

    let messages = it
        .next()
        .expect("Cannot read messages")
        .lines()
        .map(|m| m.as_bytes().to_vec())
        .collect();

    (rules, messages)
}

fn is_valid_inner(rules: &Rules, rule_number: u64, message: &[u8], pos: usize) -> Option<usize> {
    match &rules[&rule_number] {
        Rule::Match(c) if pos < message.len() && message[pos] == *c => Some(pos + 1),
        Rule::Match(_) => None,
        Rule::Composition(sub_rules) => {
            sub_rules.iter().fold(None, |position, rs| match position {
                Some(p) => Some(p),
                None => rs.iter().fold(Some(pos), |position, &r| match position {
                    None => None,
                    Some(position) => is_valid_inner(rules, r, message, position),
                }),
            })
        }
    }
}

fn is_valid(rules: &Rules, message: &[u8]) -> bool {
    is_valid_inner(&rules, 0, message, 0)
        .map(|position| position == message.len())
        .unwrap_or(false)
}

#[derive(Debug)]
enum Rule {
    Match(u8),
    Composition(Vec<Vec<u64>>),
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            s.chars()
                .nth(1)
                .map(|c| c as u8)
                .map(Rule::Match)
                .ok_or("Cannot read 2nd char for match rule".to_string())
        } else {
            let rule = s
                .split('|')
                .map(|sub_rule| {
                    sub_rule
                        .split_whitespace()
                        .filter_map(|n| n.parse().ok())
                        .collect()
                })
                .collect();
            Ok(Rule::Composition(rule))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_tests() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let (rules, messages) = parse(&input);
        assert_eq!(part_one(&rules, &messages), 2);
    }

    #[test]
    fn part_two_tests() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let (rules, messages) = parse(&input);
        assert_eq!(part_two(rules, &messages), 12);
    }
}
