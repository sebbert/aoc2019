use env_logger;
use intcode::*;
use std::iter;

fn main() {
  env_logger::init();

  let program_src = include_str!("./input");
  let program = parse_program(program_src);

  let part1 = {
    let vm = VM::new(program.clone());
    vm.run(&mut iter::once(1))
  };
  println!("Part 1: {:?}", part1);

  let part2 = {
    let vm = VM::new(program.clone());
    vm.run(&mut iter::once(5))
  };
  println!("Part 2: {:?}", part2);
}
