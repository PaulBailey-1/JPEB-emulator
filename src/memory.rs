
pub struct Memory {
  ram : Vec<u16>,   
}

impl Memory {
  pub fn new(ram_init: Vec<u16>) -> Memory {
    Memory {
        ram: ram_init
    }
  }

  pub fn read(&mut self, addr: usize) -> u16 {
    return self.ram[addr];
  }

  pub fn write(&mut self, addr: usize, data: u16) {
    self.ram[addr] = data;
  }
}