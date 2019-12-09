use env_logger;
use intcode::*;
use std::iter;

fn main() {
  env_logger::init();

  let program_src = include_str!("./input");
  let program = parse_program(program_src);

  println!("Part 1: {:?}", VM::new(program).run(&mut iter::once(1)));
  println!("Part 2: {:?}", VM::new(program).run(&mut iter::once(2)));
}
