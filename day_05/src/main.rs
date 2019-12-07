mod digits;

use digits::ReverseDigits;
use std::iter;
use log::trace;
use env_logger;

#[derive(Debug)]
struct VM<I: Iterator<Item = isize>> {
  program: Vec<isize>,
  ip: isize,
  input: I,
  output: Vec<isize>,
}

trait IO {
  fn read(&mut self, addr: Addr) -> isize;
  fn write(&mut self, addr: Addr, value: isize);
}

impl<I: Iterator<Item = isize>> IO for VM<I> {
  fn read(&mut self, addr: Addr) -> isize {
    let res = match addr {
      Addr::Abs(addr) => self.program[addr as usize],
      Addr::Imm(value) => value
    };
    trace!("      {:?} -> {}", addr, res);
    res
  }

  fn write(&mut self, addr: Addr, value: isize) {
    trace!("      {:?} <- {}", addr, value);
    match addr {
      Addr::Abs(addr) => {
        self.program[addr as usize] = value;
      }
      Addr::Imm(_) => panic!("Attempted to write to immediate value")
    }
  }
}

impl<I: Iterator<Item = isize>> VM<I> {
  fn new(program: Vec<isize>, input: impl IntoIterator<IntoIter = I, Item = isize>) -> VM<I> {
    VM {
      program,
      ip: 0,
      input: input.into_iter(),
      output: Vec::new(),
    }
  }

  fn peek_ins(&self) -> Ins {
    let &VM { ref program, ip, .. } = self;
    let first_value = self.program[self.ip as usize];
    let (opcode, addr_modes) = {
      let mut iter = ReverseDigits::new(first_value, 100);
      let opcode = iter.next().unwrap_or(0);
      let addr_mode = iter.value;
      (opcode, addr_mode)
    };

    let addr_mode_iter = ReverseDigits::new(addr_modes, 10).chain(iter::repeat(0));
    let operand_values = &program[ip as usize + 1..];

    let mut addr_iter =
      addr_mode_iter
      .zip(operand_values)
      .map(|(mode, value)| Addr::from_mode_value(mode, *value));

    let ins = match opcode {
      99 => Ins::Halt,
      01 => Ins::Add(Ops3::from_iter(addr_iter)),
      02 => Ins::Mul(Ops3::from_iter(addr_iter)),
      03 => Ins::Read(addr_iter.next().unwrap()),
      04 => Ins::Write(addr_iter.next().unwrap()),
      05 => Ins::JmpTrue(JmpOps::from_iter(addr_iter)),
      06 => Ins::JmpFalse(JmpOps::from_iter(addr_iter)),
      07 => Ins::LessThan(Ops3::from_iter(addr_iter)),
      08 => Ins::Equals(Ops3::from_iter(addr_iter)),
      _ => panic!("Invalid opcode: {}", opcode),
    };

    ins
  }

  fn run(mut self) -> Vec<isize> {
    loop {
      let ins = self.peek_ins();
      let mut next_ip = self.ip + ins.len();
      trace!("[{}] {:?}", self.ip, ins);
      match ins {
        Ins::Add(ops) => ops.apply(&mut self, |a, b| a + b),
        Ins::Mul(ops) => ops.apply(&mut self, |a, b| a * b),
        Ins::Read(target) => {
          let value = self.input.next().unwrap();
          self.write(target, value);
        }
        Ins::Write(source) => {
          let value = self.read(source);
          self.output.push(value);
        }
        Ins::JmpTrue(ops) => {
          if self.read(ops.test) != 0 {
            next_ip = self.read(ops.target);
          }
        }
        Ins::JmpFalse(ops) => {
          if self.read(ops.test) == 0 {
            next_ip = self.read(ops.target);
          }
        }
        Ins::Equals(ops) => ops.apply(&mut self, |a, b| (a == b) as isize),
        Ins::LessThan(ops) => ops.apply(&mut self, |a, b| (a < b) as isize),
        Ins::Halt => break
      }
      self.ip = next_ip;
    }

    self.output
  }
}

#[derive(Debug,Copy,Clone)]
enum Ins {
  Add(Ops3),
  Mul(Ops3),
  Read(Addr),
  Write(Addr),
  JmpTrue(JmpOps),
  JmpFalse(JmpOps),
  LessThan(Ops3),
  Equals(Ops3),
  Halt,
}

impl Ins {
  fn len(&self) -> isize {
    match self {
      Ins::Add(_)
      | Ins::Mul(_)
      | Ins::LessThan(_)
      | Ins::Equals(_)
      => 4,
      
      Ins::Read(_) | Ins::Write(_) => 2,
      Ins::JmpTrue(_) | Ins::JmpFalse(_) => 3,
      Ins::Halt => 1,
    }
  }
}

#[derive(Debug,Copy,Clone)]
enum Addr {
  Abs(isize),
  Imm(isize),
}

impl Addr {
  fn from_mode_value(mode: isize, value: isize) -> Addr {
    match mode {
      0 => Addr::Abs(value),
      1 => Addr::Imm(value),
      _ => panic!("Invalid addressing mode: {}", mode),
    }
  }

  fn expect_abs(self) -> isize {
    match self {
      Addr::Abs(addr) => addr,
      _ => panic!("Expected absolute address")
    }
  }
}

#[derive(Debug,Copy,Clone)]
struct Ops3 {
  left: Addr,
  right: Addr,
  target: Addr
}

impl Ops3 {
  fn from_iter(into_iter: impl IntoIterator<Item = Addr>) -> Ops3 {
    let mut iter = into_iter.into_iter();
    let left = iter.next().unwrap();
    let right = iter.next().unwrap();
    let target = iter.next().unwrap();
    Ops3 { left, right, target }
  }

  fn apply(&self, vm: &mut dyn IO, reduce: fn(isize, isize) -> isize) {
    let left = vm.read(self.left);
    let right = vm.read(self.right);
    let result = reduce(left, right);
    vm.write(self.target, result);
  }
}

#[derive(Debug,Copy,Clone)]
struct JmpOps {
  test: Addr,
  target: Addr,
}

impl JmpOps {
  fn from_iter(into_iter: impl IntoIterator<Item = Addr>) -> JmpOps {
    let mut iter = into_iter.into_iter();
    let test = iter.next().unwrap();
    let target = iter.next().unwrap();
    JmpOps { test, target }
  }
}

fn main() {
  env_logger::init();

  let input_str = include_str!("./input");
  let program = input_str
    .split(',')
    .map(|s| { s.trim_end().parse::<isize>().unwrap() })
    .collect::<Vec<_>>();

  let part1 = {
    let vm = VM::new(program.clone(), [1 as isize].iter().cloned());
    vm.run()
  };
  println!("Part 1: {:?}", part1);

  let part2 = {
    let vm = VM::new(program.clone(), [5 as isize].iter().cloned());
    vm.run()
  };
  println!("Part 2: {:?}", part2);
}
