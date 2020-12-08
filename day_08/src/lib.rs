use Instruction::*;
use std::fs;

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

struct CodeBranch {
    instruction: Instruction,
    next: usize,
    possible_next: usize,
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

pub fn get_acc_of_program(filename: &str) -> i32 {
    let program = fs::read_to_string(filename).unwrap();
    acc_at_loop(&program)
}

fn get_code_branch(program: &str) -> Vec<CodeBranch> {
    program.lines().enumerate().map(|(i, line)| {
        let instruction = get_instruction(line);
        let (next, possible_next) = match instruction {
            Acc(_) => (i + 1, i + 1),
            Jmp(num) => ((i as i32 + num) as usize, i + 1),
            Noop(num) => (i + 1, (i as i32 + num) as usize),
        };
        CodeBranch {
            instruction,
            next,
            possible_next,
        }
    }).collect()
}

fn get_partition_with_start(branches: &Vec<CodeBranch>, index: usize) -> Vec<usize> {
    let mut next = Some(index);
    let mut partition = Vec::new();
    while let Some(i) = next {
        if i >= branches.len() {
            break;
        }
        let branch = &branches[i];
        if partition.contains(&i) {
            next = None;
        } else {
            partition.push(i);
            next = Some(branch.next);
        }
    };
    partition
}

fn divide_into_partitions(branches: &Vec<CodeBranch>) -> Vec<Vec<usize>> {
    let mut all_visited = Vec::new();
    let mut partitions = Vec::new();
    for i in 0..branches.len() {
        if !all_visited.contains(&i) {
            let partition = get_partition_with_start(branches, i);
            all_visited.append(&mut partition.clone());
            partitions.push(partition);
        }
    }
    partitions
}

fn find_instruction_to_change(branches: &Vec<CodeBranch>, partitions: Vec<Vec<usize>>) -> usize {
    let start_partition = partitions.iter().fold(Vec::new(), |acc, partition| {
        if partition.contains(&0) {
            partition.clone()
        } else {
            acc
        }
    });
    let end_partitions: Vec<Vec<usize>> = partitions.iter().cloned().filter(|elem| elem.contains(&(branches.len() - 1))).collect();
    let mut index= 0;
    for i in start_partition.iter() {
        let mut points_to_end = false;
        let branch = &branches[*i];
        for partition in end_partitions.iter() {
            if partition.contains(&branch.possible_next) {
                points_to_end = true;
                break;
            }
        }
        if points_to_end {
            index = *i;
            break;
        }
    }
    index
}

pub fn get_acc_at_end(filename: &str) -> i32 {
    let program = fs::read_to_string(filename).unwrap();
    let code = get_code_branch(&program);
    let partitions = divide_into_partitions(&code);
    let index_to_change = find_instruction_to_change(&code, partitions);
    let mut acc = 0;
    let mut next_instruction = Some(0);
    while let Some(i) = next_instruction {
        let mut next = code[i].next;
        if i == index_to_change {
            next = code[i].possible_next;
        }
        if next < code.len() {
            next_instruction = Some(next);
        } else {
            next_instruction = None;
        }
        acc = if let Acc(num) = code[i].instruction {
            acc + num
        } else {
            acc
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use crate::{get_instruction, get_program, acc_at_loop, get_code_branch, divide_into_partitions, find_instruction_to_change, get_acc_at_end};
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

    #[test]
    fn test_find_instruction_to_change() {
        let program = fs::read_to_string("data/example.txt").unwrap();
        let branches = get_code_branch(&program);
        let partitions = divide_into_partitions(&branches);
        assert_eq!(7, find_instruction_to_change(&branches, partitions));
    }

    #[test]
    fn test_get_acc_at_end() {
        assert_eq!(8, get_acc_at_end("data/example.txt"));
    }
}
