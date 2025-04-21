use std::env;
use std::process;

pub mod graphics;
pub mod emulator;
pub mod memory;
pub mod tests;

use emulator::Emulator;

fn main() {
  let args = env::args().collect::<Vec<_>>();

  let mut datapath = "../data";
  if args.len() > 2 {
    datapath = &args[2];
  } 
  if args.len() > 1 {
    // file to run is passed as a command line argument
    let cpu = Emulator::new(&args[1], datapath);
    let result = cpu.run(true);
    println!("<< {} >>", result); // print a newline
    // process::exit(i32::from(result));
    process::exit(0);
  } else {
    println!("Usage: bemu file.bin");
    process::exit(64);
  }
}