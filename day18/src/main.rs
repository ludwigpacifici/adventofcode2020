fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part one: {:?}", part_one(&input));
    println!("part two: {:?}", part_two(&input));
}

fn part_one(s: &str) -> u64 {
    s.lines()
        .map(|l| Parser::new(l.to_string(), 1))
        .map(|mut p| p.expression())
        .map(eval)
        .sum()
}

fn part_two(s: &str) -> u64 {
    s.lines()
        .map(|l| Parser::new(l.to_string(), 2))
        .map(|mut p| p.expression())
        .map(eval)
        .sum()
}

#[derive(Debug)]
enum Expression {
    Number(u64),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

struct Parser {
    text: String,
    data: Vec<char>,
    current: usize,
    part: u8,
}

impl Parser {
    fn new(text: String, part: u8) -> Self {
        let text = text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let data = text.chars().collect::<Vec<_>>();
        let current = 0;

        Parser {
            text,
            data,
            current,
            part,
        }
    }

    fn is_end(&self) -> bool {
        self.current == self.data.len()
    }

    fn next(&mut self) -> char {
        let c = self.data[self.current];
        self.current += 1;
        c
    }

    fn number(&mut self) -> Expression {
        let end = self.data[self.current..]
            .iter()
            .position(|c| !c.is_digit(10))
            .map(|i| i + self.current)
            .unwrap_or(self.data.len());

        let n = u64::from_str_radix(&self.text[self.current..end], 10).unwrap();

        self.current = end;
        Expression::Number(n)
    }

    fn peek(&self) -> char {
        self.data[self.current]
    }

    fn parentheses(&mut self) -> Expression {
        if self.peek() == '(' {
            self.next();
            let expr = self.expression();
            debug_assert!(self.peek() == ')');
            self.next();
            expr
        } else {
            self.number()
        }
    }

    fn addition_or_multiplication(&mut self) -> Expression {
        let mut expression = self.parentheses();

        while !self.is_end() && (self.peek() == '*' || self.peek() == '+') {
            let c = self.next();
            if c == '*' {
                let rhs = self.parentheses();
                expression = Expression::Multiplication(Box::new(expression), Box::new(rhs))
            } else if c == '+' {
                let rhs = self.parentheses();
                expression = Expression::Addition(Box::new(expression), Box::new(rhs))
            }
        }

        expression
    }

    fn addition(&mut self) -> Expression {
        let mut expression = self.parentheses();

        while !self.is_end() && self.peek() == '+' {
            let c = self.next();
            if c == '+' {
                let rhs = self.parentheses();
                expression = Expression::Addition(Box::new(expression), Box::new(rhs))
            }
        }

        expression
    }

    fn multiplication(&mut self) -> Expression {
        let mut expression = self.addition();

        while !self.is_end() && self.peek() == '*' {
            let c = self.next();
            if c == '*' {
                let rhs = self.addition();
                expression = Expression::Multiplication(Box::new(expression), Box::new(rhs))
            }
        }

        expression
    }

    fn expression(&mut self) -> Expression {
        if self.part == 1 {
            // expression -> addition_or_multiplication
            // addition_or_multiplication -> parentheses (("+" | "*") parentheses)*
            // parentheses -> "(" expression ")" | digit
            // digit -> [0..9]+
            self.addition_or_multiplication()
        } else {
            // expression -> multiplication
            // multiplication -> addition ("*" addition)*
            // addition -> parentheses ("+" parentheses)*
            // parentheses -> "(" expression ")" | digit
            // digit -> [0..9]+
            self.multiplication()
        }
    }
}

fn eval(expr: Expression) -> u64 {
    match expr {
        Expression::Number(n) => n,
        Expression::Addition(l, r) => eval(*l) + eval(*r),
        Expression::Multiplication(l, r) => eval(*l) * eval(*r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_tests() {
        assert_eq!(part_one("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(part_one("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_one("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn part_two_tests() {
        assert_eq!(part_two("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(part_two("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_two("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
