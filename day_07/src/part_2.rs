use intcode::*;
use permutohedron::Heap;
use std::iter;

fn run_amps(program: &Vec<isize>, phase_settings: &[isize]) -> isize {
  let mut amps = 
    phase_settings.iter()
    .map(|&phase| {
      let mut vm = VM::new(program.clone());
      let mut phase_input = iter::once(phase);
      vm.run_until_next_input(&mut phase_input);
      vm
    })
    .collect::<Vec<_>>();

  let mut signal = 0;
  loop {
    let mut next_signal = signal;
    for amp in amps.iter_mut() {
      next_signal = match amp.run_until_next_output(&mut iter::once(next_signal)) {
        Some(output) => output,
        None => return signal
      }
    }
    signal = next_signal;
  }
}

pub fn part_2(program: &Vec<isize>) -> isize {
  let mut phase_settings = [5,6,7,8,9];
  Heap::new(&mut phase_settings)
    .map(|settings| run_amps(&program, &settings))
    .max()
    .unwrap()
}
