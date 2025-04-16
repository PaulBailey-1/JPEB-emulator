use std::env;
use std::process;

pub mod graphics;
pub mod emulator;
pub mod memory;
pub mod graphics;
pub mod tests;

use emulator::Emulator;
use graphics::main_;

fn main() {
  let args = env::args().collect::<Vec<_>>();

  if args.len() > 2 {
    println!("Usage: bemu file.bin");
    process::exit(64);
  } else if args.len() == 2 {
    // file to run is passed as a command line argument
    let cpu = Emulator::new(args[1].clone());
    let result = cpu.run(true);
    println!(""); // print a newline
    process::exit(i32::from(result));
  } else {
    main_();
  }
}