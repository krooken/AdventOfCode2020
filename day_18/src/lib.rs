use regex::{Regex, CaptureMatches};
use crate::Symbol::Lparen;
use std::fs;

#[derive(PartialEq)]
enum Symbol {
    Num(i64),
    Plus,
    Times,
    Lparen,
}

struct ParseTree {
    symbol: Symbol,
    left: Option<Box<ParseTree>>,
    right: Option<Box<ParseTree>>,
}

impl ParseTree {

    fn add_symbol(mut self, symbol: Symbol) -> ParseTree {
        match symbol {
            Symbol::Times => {
                ParseTree {
                    symbol,
                    left: Some(Box::new(self)),
                    right: None
                }
            },
            Symbol::Plus => {
                if self.symbol == Symbol::Times {
                    self.right = Some(Box::new(ParseTree{
                        symbol,
                        left: self.right,
                        right: None,
                    }));
                    self
                } else {
                    ParseTree {
                        symbol,
                        left: Some(Box::new(self)),
                        right: None,
                    }
                }
            },
            _ => panic!(),
        }
    }

    fn add_tree(&mut self, tree: ParseTree) {
        match &self.symbol {
            Symbol::Plus
            | Symbol::Times => {
                match &mut self.right {
                    Some(t) => t.add_tree(tree),
                    None => self.right = Some(Box::new(tree)),
                }
            },
            Symbol::Lparen => {
                match &self.left {
                    None => self.left = Some(Box::new(tree)),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    fn calculate(&self) -> i64 {
        match self.symbol {
            Symbol::Times => self.left.as_ref().unwrap().calculate() * self.right.as_ref().unwrap().calculate(),
            Symbol::Plus => self.left.as_ref().unwrap().calculate() + self.right.as_ref().unwrap().calculate(),
            Symbol::Lparen => self.left.as_ref().unwrap().calculate(),
            Symbol::Num(num) => num,
        }
    }
}

fn push_num(stack: &mut Vec<Symbol>, num: i64) {
    match stack.pop() {
        Some(x) => {
            match x {
                Symbol::Lparen => {
                    stack.push(x);
                    stack.push(Symbol::Num(num));
                },
                Symbol::Plus => {
                    if let Some(Symbol::Num(n)) = stack.pop() {
                        stack.push(Symbol::Num(n + num));
                    } else {
                        panic!();
                    }
                },
                Symbol::Times => {
                    if let Some(Symbol::Num(n)) = stack.pop() {
                        stack.push(Symbol::Num(n * num));
                    } else {
                        panic!();
                    }
                },
                _ => panic!(),
            }
        }
        None => {
            stack.push(Symbol::Num(num));
        }
    }
}

fn calculate_row(row: &str) -> i64 {
    let re = Regex::new(r"(\d+|\(|\)|\+|\*)").unwrap();
    let mut stack = Vec::new();
    let caps = re.captures_iter(row);
    for cap in caps {
        match &cap[0] {
            "(" => stack.push(Lparen),
            ")" => {
                if let Some(Symbol::Num(n)) = stack.pop() {
                    stack.pop().unwrap();
                    push_num(&mut stack, n);
                } else {
                    panic!();
                }
            },
            "+" => stack.push(Symbol::Plus),
            "*" => stack.push(Symbol::Times),
            num => {
                let i = num.parse().unwrap();
                push_num(&mut stack, i);
            }
        }
    }
    if let Some(Symbol::Num(num)) = stack.last() {
        *num
    } else {
        panic!();
    }
}

pub fn sum_expressions(filename: &str) -> i64 {
    let text = fs::read_to_string(filename).unwrap();
    text.lines().map(|line| calculate_row(line)).sum()
}

fn parse_expression(mut captures: &mut CaptureMatches) -> ParseTree {
    let cap = captures.next().unwrap();
    let mut tree = match &cap[0] {
        "(" => {
            ParseTree {
                symbol: Symbol::Lparen,
                left: Some(Box::new(parse_expression(&mut captures))),
                right: None,
            }
        },
        ")"
        | "+"
        | "*" => panic!(),
        num_str => {
            ParseTree {
                symbol: Symbol::Num(num_str.parse().unwrap()),
                left: None,
                right: None,
            }
        },
    };
    while let Some(cap) = captures.next() {
        match &cap[0] {
            "(" => tree.add_tree(ParseTree {
                symbol: Symbol::Lparen,
                left: Some(Box::new(parse_expression(&mut captures))),
                right: None,
            }),
            ")" => return tree,
            "+" => tree = tree.add_symbol(Symbol::Plus),
            "*" => tree = tree.add_symbol(Symbol::Times),
            num_str => {
                let num = num_str.parse().unwrap();
                tree.add_tree(ParseTree {
                    symbol: Symbol::Num(num),
                    left: None,
                    right: None,
                });
            },
        }
    }
    tree
}

fn parse_row(text: &str) -> ParseTree {
    let re = Regex::new(r"(\d+|\(|\)|\+|\*)").unwrap();
    let mut captures = re.captures_iter(text);
    parse_expression(&mut captures)
}

fn calculate_row_precedence(text: &str) -> i64 {
    parse_row(text).calculate()
}

pub fn sum_expressions_precedence(filename: &str) -> i64 {
    let text = fs::read_to_string(filename).unwrap();
    text.lines().map(|line| {
        calculate_row_precedence(line)
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_row, sum_expressions, calculate_row_precedence, sum_expressions_precedence};

    #[test]
    fn test_calculate_row() {
        assert_eq!(71, calculate_row("1 + 2 * 3 + 4 * 5 +6"));
    }

    #[test]
    fn test_calculate_row_paren() {
        assert_eq!(51, calculate_row("1 + (2 * 3) + (4 * (5 +6))"));
    }

    #[test]
    fn test_sum_expressions() {
        assert_eq!(26+437+12240+13632, sum_expressions("data/example.txt"));
    }

    #[test]
    fn test_calculate_row_precedence() {
        assert_eq!(231, calculate_row_precedence("1 + 2 * 3 + 4 *5 + 6"));
    }

    #[test]
    fn test_calculate_row_precedence_paren() {
        assert_eq!(51, calculate_row_precedence("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn test_sum_expressions_precedence() {
        assert_eq!(46 + 1445 + 669_060 + 23_340, sum_expressions_precedence("data/example.txt"));
    }

    #[test]
    fn test_task1() {
        assert_eq!(2743012121210, sum_expressions("data/math.txt"));
    }

    #[test]
    fn test_task2() {
        assert_eq!(65658760783597, sum_expressions_precedence("data/math.txt"));
    }
}
