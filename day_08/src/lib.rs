
#[derive(Debug, PartialEq)]
enum Instruction {
    Noop(i32),
    Acc(i32),
    Jmp(i32),
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

#[cfg(test)]
mod tests {
    use crate::get_instruction;
    use crate::Instruction::{Acc, Jmp, Noop};

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
}
