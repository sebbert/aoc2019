use intcode::*;
use permutohedron::Heap;

fn run_amp(program: &Vec<isize>, signal: isize, phase: isize) -> isize {
  let program = program.clone();
  let input = [phase, signal];
  let input = input.iter().cloned();
  let vm = VM::new(program, input);
  *vm.run().last().unwrap()
}

fn run_amps(program: &Vec<isize>, phase_settings: &[isize]) -> isize {
  phase_settings
    .iter()
    .fold(0, |signal, &phase| run_amp(program, signal, phase))
}

pub fn part_1(program: &Vec<isize>) -> isize {
  let mut phase_settings = [0,1,2,3,4];
  Heap::new(&mut phase_settings)
    .map(|settings| run_amps(&program, &settings))
    .max()
    .unwrap()
}
