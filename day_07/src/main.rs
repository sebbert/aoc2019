use env_logger;
use intcode::*;

mod part_1;
mod part_2;

use part_1::*;
use part_2::*;

fn main() {
  env_logger::init();

  let program_src = include_str!("./input");
  let program = parse_program(program_src);

  println!("Part 1: {}", part_1(&program));
  println!("Part 2: {}", part_2(&program));
}
