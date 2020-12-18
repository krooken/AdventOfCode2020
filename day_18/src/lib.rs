use regex::Regex;
use crate::Symbol::Lparen;

enum Symbol {
    Num(i64),
    Plus,
    Times,
    Lparen,
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

#[cfg(test)]
mod tests {
    use crate::calculate_row;

    #[test]
    fn test_calculate_row() {
        assert_eq!(71, calculate_row("1 + 2 * 3 + 4 * 5 +6"));
    }

    #[test]
    fn test_calculate_row_paren() {
        assert_eq!(51, calculate_row("1 + (2 * 3) + (4 * (5 +6))"));
    }
}
