
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
        "acc" => Instruction::Acc(num),
        "jmp" => Instruction::Jmp(num),
        "nop" => Instruction::Noop(num),
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

#[cfg(test)]
mod tests {
    use crate::{get_instruction, get_program};
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
}
