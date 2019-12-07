use env_logger;
use intcode::*;

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
