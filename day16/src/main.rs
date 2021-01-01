use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = input.parse::<Input>().expect("Cannot parse the input");

    println!(
        "part one: {:?}",
        part_one(&input.nearby_tickets, &input.rules)
    );
    println!("part two: {:?}", part_two(&input));

    Ok(())
}

fn part_one(nearby_tickets: &[Ticket], rules: &[Rule]) -> u64 {
    nearby_tickets
        .iter()
        .map(|ticket| errors(ticket, rules).into_iter().sum::<u64>())
        .sum()
}

fn part_two(input: &Input) -> u64 {
    let order = find_field_order(&input)
        .into_iter()
        .enumerate()
        .filter_map(|(rule_order, rule_number)| {
            if input.rules[rule_number].name.starts_with("departure") {
                Some(rule_order)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    order.iter().map(|&i| input.your_ticket[i]).product()
}

struct Input {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("\n\n");
        let rules = s.next().ok_or("Cannot get the rules from the input")?;

        let rules = rules
            .lines()
            .map(|l| l.parse::<Rule>())
            .collect::<Result<_, _>>()?;

        let your_ticket = s.next().ok_or("Cannot get your ticket from the input")?;

        let your_ticket = your_ticket
            .lines()
            .skip(1)
            .flat_map(|l| l.split(','))
            .map(|n| n.parse::<u64>().map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;

        let nearby_tickets = s.next().ok_or("Cannot get nearby tickets from the input")?;

        let nearby_tickets = nearby_tickets
            .lines()
            .skip(1)
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse::<u64>().expect("Cannot read ticket details"))
                    .collect::<Ticket>()
            })
            .collect();

        Ok(Input {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

type Ticket = Vec<u64>;

type TicketRule = (RangeInclusive<u64>, RangeInclusive<u64>);

fn is_ticket_rule_valid(rule: &TicketRule, v: &u64) -> bool {
    rule.0.contains(v) || rule.1.contains(v)
}

struct Rule {
    name: String,
    ranges: TicketRule,
}

impl std::str::FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = Regex::new(r"^([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        rule.captures(s)
            .map(|captures| {
                let name = captures[1].to_string();
                let first_begin = parse_u64(&captures[2])?;
                let first_end = parse_u64(&captures[3])?;
                let second_begin = parse_u64(&captures[4])?;
                let second_end = parse_u64(&captures[5])?;
                let ranges = (
                    RangeInclusive::new(first_begin, first_end),
                    RangeInclusive::new(second_begin, second_end),
                );
                Ok(Rule { name, ranges })
            })
            .unwrap_or(Err("Regex captured nothing".to_string()))
    }
}

fn parse_u64(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|e| e.to_string())
}

fn errors<'a>(ticket: &'a Ticket, rules: &[Rule]) -> Vec<&'a u64> {
    ticket
        .into_iter()
        .filter(|&field| {
            !rules
                .iter()
                .any(|rule| is_ticket_rule_valid(&rule.ranges, field))
        })
        .collect()
}

fn ok_tickets(nearby_tickets: &[Ticket], rules: &[Rule]) -> Vec<Ticket> {
    nearby_tickets
        .iter()
        .filter(|ticket| errors(ticket, rules).is_empty())
        .cloned()
        .collect()
}

fn valid_rules_per_field(rules: &[Rule], ok_tickets: &[Ticket]) -> Vec<HashSet<usize>> {
    let len = rules.len();
    (0..len).fold(Vec::new(), |mut all_valid_rules, field_number| {
        let current_valid_rules = ok_tickets.iter().map(|t| t[field_number]).fold(
            (0..len).collect::<HashSet<usize>>(),
            |valid_rules, field| {
                let valid_rule_for_one_field = (0..len)
                    .filter(|&rule_number| is_ticket_rule_valid(&rules[rule_number].ranges, &field))
                    .collect::<HashSet<usize>>();

                valid_rules
                    .intersection(&valid_rule_for_one_field)
                    .cloned()
                    .collect()
            },
        );

        all_valid_rules.push(current_valid_rules);
        all_valid_rules
    })
}

fn reduce_field_order(field_possibilities: &[HashSet<usize>]) -> Vec<usize> {
    let mut fields_order = vec![];
    for i in 0..field_possibilities.len() {
        let mut diff = field_possibilities[i].clone();
        for j in 0..field_possibilities.len() {
            if field_possibilities[j].len() < field_possibilities[i].len() {
                diff = diff
                    .difference(&field_possibilities[j])
                    .cloned()
                    .collect::<HashSet<usize>>();
            }
        }
        debug_assert!(diff.len() == 1);
        let remainder = *diff.iter().next().unwrap();
        fields_order.push(remainder);
    }

    debug_assert!(fields_order.len() == 20);
    fields_order
}

fn find_field_order(input: &Input) -> Vec<usize> {
    let ok_tickets = ok_tickets(&input.nearby_tickets, &input.rules);
    let field_possibilities = valid_rules_per_field(&input.rules, &ok_tickets);
    reduce_field_order(&field_possibilities)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let input = input.parse::<Input>().unwrap();
        assert_eq!(part_one(&input.nearby_tickets, &input.rules), 71)
    }
}
