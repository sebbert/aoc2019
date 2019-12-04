#[derive(Debug)]
enum Ins {
    Add(Operands),
    Mul(Operands),
    Halt,
}

#[derive(Debug)]
struct Operands {
    left: usize,
    right: usize,
    target: usize
}

fn parse_operands(program: &[usize], ip: usize) -> Operands {
    Operands {
        left: program[ip+1],
        right: program[ip+2],
        target: program[ip+3],
    }
}

fn parse_ins(program: &[usize], ip: usize) -> Ins {
    let op = program[ip];
    match op {
        1 => Ins::Add(parse_operands(program, ip)),
        2 => Ins::Mul(parse_operands(program, ip)),
        99 => Ins::Halt,
        _ => panic!("Unknown opcode: {}", op),
    }
}

fn apply_ins(program: &mut[usize], operands: Operands, reduce: fn(usize, usize) -> usize) {
    let left = program[operands.left];
    let right = program[operands.right];
    program[operands.target] = reduce(left, right);
}

fn run_program(program: &mut[usize]) {
    let mut ip = 0;
    loop {
        let ins = parse_ins(&program, ip);

        match ins {
            Ins::Halt => break,
            Ins::Add(operands) => apply_ins(program, operands, |a, b| a + b),
            Ins::Mul(operands) => apply_ins(program, operands, |a, b| a * b),
        }

        ip += 4;
    }
}

fn get_result_with_params(program: &mut[usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;

    run_program(program);

    program[0]
}

fn part1(input_program: &Vec<usize>) {
    let mut program = input_program.clone();
    let result = get_result_with_params(&mut program, 12, 2);
    println!("Part 1: {}", result);
}

fn part2(input_program: &Vec<usize>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_program.clone();
            let result = get_result_with_params(&mut program, noun, verb);
            if result == 19690720 {
                println!("Part 2: {}", 100*noun + verb);
                return;
            }
        }
    }
}

fn main() {
    let input_str = include_str!("./input");
    let program = input_str
        .split(',')
        .map(|s| { s.trim_end().parse::<usize>().unwrap() })
        .collect::<Vec<_>>();

    part1(&program);
    part2(&program);
}
