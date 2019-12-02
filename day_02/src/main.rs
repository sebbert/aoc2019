#[derive(Debug)]
enum Ins {
    Add(Operands),
    Mul(Operands),
    Halt,
}

impl Ins {
    fn num_values(&self) -> usize {
        match self {
            &Ins::Add(ref ops) => 1 + ops.num_values(),
            &Ins::Mul(ref ops) => 1 + ops.num_values(),
            &Ins::Halt => 1,
        }
    }
}

#[derive(Debug)]
struct Operands {
    left: usize,
    right: usize,
    target: usize
}

impl Operands {
    fn num_values(&self) -> usize { 3 }
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

fn main() {
    let input_str = include_str!("./input");
    let input_program = input_str
        .split(',')
        .map(|s| { s.trim_end().parse::<usize>().unwrap() })
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_program.clone();
            program[1] = noun;
            program[2] = verb;
        
            let mut ip = 0;
            loop {
                let ins = parse_ins(&program, ip);
                let num_values = ins.num_values();
        
                match ins {
                    Ins::Halt => break,
                    Ins::Add(operands) => apply_ins(&mut program, operands, |a, b| a + b),
                    Ins::Mul(operands) => apply_ins(&mut program, operands, |a, b| a * b),
                }
        
                ip += num_values;
            }
            
            let result = program[0];
            if result == 19690720 {
                println!("{}", 100*noun + verb);
                return;
            }
        }
    }

}
