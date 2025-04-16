  movi r3, 0xE000
  swi r6, r3, 0x0101
  addi r3, r3, 1
  swi r6, r3, 0x0101
  movi r3, 0xE028 # 0xE000 + 40 (0x28)
  swi r6, r3, 0x0001
  addi r3, r3, 1
  swi r6, r3, 0x0100
  movi r3, 0xE050 # 0xE000 + 80 (0x50)
  swi r6, r3, 0x0001
  addi r3, r3, 1
  swi r6, r3, 0x0100
  movi r3, 0xE078 # 0xE000 + 120 (0x78)
  swi r6, r3, 0x0101
  addi r3, r3, 1
  swi r6, r3, 0x0101
  movi r7, 50000
STALL_BEGIN:
  addi r7, r7, -1
  bnz STALL_BEGIN