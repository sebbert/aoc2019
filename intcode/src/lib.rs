mod digits;

use digits::ReverseDigits;
use std::iter;
use log::trace;

#[derive(Debug)]
pub struct VM {
  program: Vec<isize>,
  ip: isize,
  output: Vec<isize>,
}

pub trait IO {
  fn read(&mut self, addr: Addr) -> isize;
  fn write(&mut self, addr: Addr, value: isize);
}

impl IO for VM {
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

impl VM {
  pub fn new(program: Vec<isize>) -> VM {
    VM {
      program,
      ip: 0,
      output: Vec::new(),
    }
  }

  pub fn peek_ins(&self) -> Ins {
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

  pub fn step(&mut self, input: &mut dyn Iterator<Item = isize>) -> Ins {
    let ins = self.peek_ins();
    let mut next_ip = self.ip + ins.len();
    trace!("[{}] {:?}", self.ip, ins);
    match ins {
      Ins::Add(ops) => ops.apply(self, |a, b| a + b),
      Ins::Mul(ops) => ops.apply(self, |a, b| a * b),
      Ins::Read(target) => {
        let value = input.next().unwrap();
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
      Ins::Equals(ops) => ops.apply(self, |a, b| (a == b) as isize),
      Ins::LessThan(ops) => ops.apply(self, |a, b| (a < b) as isize),
      Ins::Halt => {
        // Rewind to start of halt instruction
        next_ip = self.ip;
      }
    }
    self.ip = next_ip;

    ins
  }

  pub fn run_until_next_output(&mut self, input: &mut dyn Iterator<Item = isize>) -> Option<isize> {
    loop {
      match self.step(input) {
        Ins::Write(from_addr) => {
          // Not super stoked about reading the value twice here, but it's okay for now
          return Some(self.read(from_addr))
        }
        Ins::Halt => return None,
        _ => {}
      }
    }
  }

  pub fn run_until_next_input(&mut self, input: &mut dyn Iterator<Item = isize>) {
    loop {
      match self.step(input) {
        Ins::Read(_) | Ins::Halt => return,
        _ => {}
      }
    }
  }

  pub fn run(mut self, input: &mut dyn Iterator<Item = isize>) -> Vec<isize> {
    loop {
      match self.step(input) {
        Ins::Halt => break,
        _ => {}
      }
    }

    self.output
  }
}

#[derive(Debug,Copy,Clone)]
pub enum Ins {
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
  pub fn len(&self) -> isize {
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
pub enum Addr {
  Abs(isize),
  Imm(isize),
}

impl Addr {
  pub fn from_mode_value(mode: isize, value: isize) -> Addr {
    match mode {
      0 => Addr::Abs(value),
      1 => Addr::Imm(value),
      _ => panic!("Invalid addressing mode: {}", mode),
    }
  }
}

#[derive(Debug,Copy,Clone)]
pub struct Ops3 {
  pub left: Addr,
  pub right: Addr,
  pub target: Addr
}

impl Ops3 {
  pub fn from_iter(into_iter: impl IntoIterator<Item = Addr>) -> Ops3 {
    let mut iter = into_iter.into_iter();
    let left = iter.next().unwrap();
    let right = iter.next().unwrap();
    let target = iter.next().unwrap();
    Ops3 { left, right, target }
  }

  pub fn apply(&self, vm: &mut dyn IO, reduce: fn(isize, isize) -> isize) {
    let left = vm.read(self.left);
    let right = vm.read(self.right);
    let result = reduce(left, right);
    vm.write(self.target, result);
  }
}

#[derive(Debug,Copy,Clone)]
pub struct JmpOps {
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

pub fn parse_program(program_src: &str) -> Vec<isize> {
  program_src
    .trim_end()
    .split(',')
    .map(|s| { s.parse::<isize>().unwrap() })
    .collect::<Vec<_>>()
}
