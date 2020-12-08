use Instruction::*;

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop(i32),
    Acc(i32),
    Jmp(i32),
}

struct CodeLine {
    instruction: Instruction,
    visited: bool,
}

fn get_instruction(line: &str) -> Instruction {
    let re = regex::Regex::new(r"^(acc|jmp|nop) ((\+|-)\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    let num = cap[2].parse().unwrap();
    match &cap[1] {
        "acc" => Acc(num),
        "jmp" => Jmp(num),
        "nop" => Noop(num),
        _ => panic!(),
    }
}

fn get_program(program: &str) -> Vec<CodeLine> {
    program.lines().map(|line| {
        let instruction = get_instruction(line);
        CodeLine {
            instruction,
            visited: false,
        }
    }).collect()
}

pub fn acc_at_loop(program: &str) -> i32 {
    let mut code = get_program(program);
    let mut acc = 0;
    let mut next_instruction: Option<usize> = Some(0);
    while next_instruction.is_some() {
        let index = next_instruction.unwrap();
        let mut current_code = code.get_mut(index).unwrap();
        if current_code.visited {
            next_instruction = None;
        } else {
            match current_code.instruction {
                Acc(num) => {
                    acc += num;
                    next_instruction = Some(index + 1);
                },
                Jmp(num) => next_instruction = Some((index as i32 + num) as usize),
                Noop(_) => next_instruction = Some(index + 1),
            };
            current_code.visited = true;
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use crate::{get_instruction, get_program, acc_at_loop};
    use crate::Instruction::{Acc, Jmp, Noop};
    use std::fs;

    #[test]
    fn test_get_instruction_acc() {
        assert_eq!(Acc(4), get_instruction("acc +4"));
    }

    #[test]
    fn test_get_instruction_jmp() {
        assert_eq!(Jmp(-10), get_instruction("jmp -10"));
    }

    #[test]
    fn test_get_instruction_nop() {
        assert_eq!(Noop(0), get_instruction("nop +0"));
    }

    #[test]
    fn test_get_program() {
        let program = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(Noop(0), get_program(&program)[0].instruction);
        assert_eq!(Acc(6), get_program(&program)[8].instruction);
        assert!(!get_program(&program)[0].visited);
    }

    #[test]
    fn test_acc_at_loop() {
        let program = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(5, acc_at_loop(&program));
    }
}
