use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let bags = parse(&input);

    println!("part one: {:?}", part_one(&bags));
    println!("part two: {:?}", part_two(&bags));

    Ok(())
}

fn part_one(bags: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    bags.keys()
        .filter(|&c| *c != "shiny gold")
        .filter(|c| eventually_contain_at_least_one_shiny_gold_bag(&bags, c))
        .count()
}

// Fast enough without memoization
fn eventually_contain_at_least_one_shiny_gold_bag(
    bags: &HashMap<&str, Vec<(usize, &str)>>,
    color: &str,
) -> bool {
    if bags.contains_key(color) && color == "shiny gold" {
        true
    } else {
        bags.get(color)
            .expect(format!("No entry for color {:?}.", color).as_str())
            .iter()
            .any(|(_, children_color)| {
                eventually_contain_at_least_one_shiny_gold_bag(&bags, children_color)
            })
    }
}

fn part_two(bags: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    count_inside_bags(&bags, "shiny gold")
}

// Fast enough without memoization
fn count_inside_bags(bags: &HashMap<&str, Vec<(usize, &str)>>, color: &str) -> usize {
    if bags
        .get(color)
        .expect(format!("No entry for color {:?}.", color).as_str())
        .is_empty()
    {
        0
    } else {
        bags.get(color)
            .expect(format!("No entry for color {:?}.", color).as_str())
            .iter()
            .map(|(count, children_color)| count + count * count_inside_bags(&bags, children_color))
            .sum()
    }
}

fn parse(s: &str) -> HashMap<&str, Vec<(usize, &str)>> {
    // Captures the color at the beginning of the sentence.
    let container = Regex::new(r"^(\w+ \w+) bags contain.*$").unwrap();
    // Captures the number and colors after 'bags contain ...'.
    let children = Regex::new(r"(\d) (\w+ \w+)*").unwrap();

    s.lines()
        .map(|l| {
            let current = container
                .captures(l)
                .expect("Cannot get captures")
                .get(1)
                .expect("Cannot get current bag capture")
                .as_str();
            let children = children
                .captures_iter(l)
                .map(|captures| {
                    (
                        captures[1].parse::<usize>().expect("Cannot parse number"),
                        captures.get(2).expect("Cannot read color").as_str(),
                    )
                })
                .collect();
            (current, children)
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        assert_eq!(part_one(&parse(input)), 4);
    }

    #[test]
    fn part_two_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        assert_eq!(part_two(&parse(input)), 32);

        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
        assert_eq!(part_two(&parse(input)), 126);
    }
}
