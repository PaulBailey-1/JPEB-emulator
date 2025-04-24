#[cfg(test)]

use super::*;

const DATA_PATH: &str = "../data";

#[test]
fn addi_test() {
  let cpu = Emulator::new("../tests/bin/addi_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 14);
}

#[test]
fn sw_lw_test() {
  let cpu = Emulator::new("../tests/bin/sw_lw_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 42);
}

#[test]
fn swi_test() {
  let cpu = Emulator::new("../tests/bin/swi_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 15);
}

#[test]
fn lui_test() {
  let cpu = Emulator::new("../tests/bin/lui_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 512);
}

#[test]
fn movi_test() {
  let cpu = Emulator::new("../tests/bin/movi_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 513);
}

#[test]
fn jalr_test() {
  let cpu = Emulator::new("../tests/bin/jalr_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 42);
}

#[test]
fn nand_test() {
  let cpu = Emulator::new("../tests/bin/nand_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result as i16, -3);
}

#[test]
fn add_test() {
  let cpu = Emulator::new("../tests/bin/add_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 42);
}

#[test]
fn addc_test() {
  let cpu = Emulator::new("../tests/bin/addc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0xAAAC);
}

#[test]
fn or_test() {
  let cpu = Emulator::new("../tests/bin/or_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 14);
}

#[test]
fn subc_test() {
  let cpu = Emulator::new("../tests/bin/subc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0xFFFF);
}

#[test]
fn and_test() {
  let cpu = Emulator::new("../tests/bin/and_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 2);
}

#[test]
fn sub_test() {
  let cpu = Emulator::new("../tests/bin/sub_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result as i16, -7);
}

#[test]
fn xor_test() {
  let cpu = Emulator::new("../tests/bin/xor_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 29);
}
#[test]
fn not_test() {
  let cpu = Emulator::new("../tests/bin/not_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 2);
}

#[test]
fn shl_test() {
  let cpu = Emulator::new("../tests/bin/shl_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x5554);
}
#[test]
fn shr_test() {
  let cpu = Emulator::new("../tests/bin/shr_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x2AAA);
}

#[test]
fn rotl_test() {
  let cpu = Emulator::new("../tests/bin/rotl_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x5555);
}

#[test]
fn rotr_test() {
  let cpu = Emulator::new("../tests/bin/rotr_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0xAAAA);
}

#[test]
fn sshr_test() {
  let cpu = Emulator::new("../tests/bin/sshr_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0xD555);
}

#[test]
fn shrc_test() {
  let cpu = Emulator::new("../tests/bin/shrc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x8050);
}

#[test]
fn shlc_test() {
  let cpu = Emulator::new("../tests/bin/shlc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x00A1);
}

#[test]
fn beq_test() {
  let cpu = Emulator::new("../tests/bin/beq_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bp_test() {
  let cpu = Emulator::new("../tests/bin/bp_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bn_test() {
  let cpu = Emulator::new("../tests/bin/bn_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bc_test() {
  let cpu = Emulator::new("../tests/bin/bc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bo_test() {
  let cpu = Emulator::new("../tests/bin/bo_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bne_test() {
  let cpu = Emulator::new("../tests/bin/bne_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn jmp_test() {
  let cpu = Emulator::new("../tests/bin/jmp_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bnc_test() {
  let cpu = Emulator::new("../tests/bin/bnc_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bg_test() {
  let cpu = Emulator::new("../tests/bin/bg_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bge_test() {
  let cpu = Emulator::new("../tests/bin/bge_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bl_test() {
  let cpu = Emulator::new("../tests/bin/bl_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn ble_test() {
  let cpu = Emulator::new("../tests/bin/ble_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn ba_test() {
  let cpu = Emulator::new("../tests/bin/ba_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bae_test() {
  let cpu = Emulator::new("../tests/bin/bae_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bb_test() {
  let cpu = Emulator::new("../tests/bin/bb_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn bbe_test() {
  let cpu = Emulator::new("../tests/bin/bbe_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0);
}

#[test]
fn collatz_test() {
  let cpu = Emulator::new("../tests/bin/collatz_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 9232);
}

#[test]
fn load_test() {
  let cpu = Emulator::new("../tests/bin/load_test.bin", DATA_PATH);
  let result = cpu.run(false);
  assert_eq!(result, 0x0FFF);
}