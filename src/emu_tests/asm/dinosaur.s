# note that this follows the following ISA convensions
# r1 = stack pointer
# r7 = return link

INIT:
  movi r3, 0x0000
  movi r4, 0x05A4
  call write_text_tilemap
  # color the background to green (tile number 0)
  movi r3, 0
  movi r4, 0x05A4
  call write_solid_tile
  # color the snake yellow (tile number 1)
  movi r3, 1
  movi r4, 0x1FF
  call write_solid_tile
  # design the apple red (tile number 4)
  movi r4, 0xC100
  movi r3, 0x01F1
  sw r3, r4, 4
  sw r3, r4, 5
  addi r4, r4, 8
  sw r3, r4, 3
  movi r3, 0x011F
  sw r3, r4, 2
  sw r3, r4, 4
  sw r3, r4, 5
  addi r4, r4, 8
  sw r3, r4, 1
  sw r3, r4, 2
  sw r3, r4, 3
  sw r3, r4, 4
  sw r3, r4, 5
  sw r3, r4, 6
  addi r4, r4, 8
  sw r3, r4, 1
  sw r3, r4, 2
  sw r3, r4, 3
  sw r3, r4, 4
  sw r3, r4, 5
  sw r3, r4, 6
  addi r4, r4, 8
  sw r3, r4, 1
  sw r3, r4, 2
  sw r3, r4, 3
  sw r3, r4, 4
  sw r3, r4, 5
  sw r3, r4, 6
  addi r4, r4, 8
  sw r3, r4, 1
  sw r3, r4, 2
  sw r3, r4, 3
  sw r3, r4, 4
  sw r3, r4, 5
  sw r3, r4, 6
  addi r4, r4, 8
  sw r3, r4, 2
  sw r3, r4, 3
  sw r3, r4, 4
  sw r3, r4, 5
  call clear_screen
JPEB_DINO_RUN:
  movi r3, TJPEB_DINO_RUN
  call print
  # increase scale to see text
  movi r4, 0xFFFC
  movi r3, 2
  sw r3, r4, 0
PRESS_SPACE_TO_START:
  movi r3, TPRESS_SPACE_TO_START
  call print
  # increase scale to see text
  movi r4, 0xFFFC
  movi r3, 2
  sw r3, r4, 0
LPRESS_SPACE_TO_START:
  movi r4, 0xFFFF
  lw r4, r4, 0
  movi r3, 0x20
  cmp r4, r3
  bne LPRESS_SPACE_TO_START
  call clear_screen
    # Draw a line at Y = 5
  # restore scale 
  movi r4, 0xFFFC
  sw r0, r4, 0

  movi r4, 64     # Set Y position (e.g., row 5)
  call draw_line 

  sys EXIT

# data things 
TJPEB_DINO_RUN:
  .fill 0x4A
  .fill 0x50
  .fill 0x45
  .fill 0x42
  .fill 0x20
  .fill 0x44
  .fill 0x69
  .fill 0x6E
  .fill 0x6F
  .fill 0x20
  .fill 0x52
  .fill 0x75
  .fill 0x6E
  .fill 0x0A
  .fill 0x00

TPRESS_SPACE_TO_START:
  .fill 0x50
  .fill 0x52
  .fill 0x45
  .fill 0x53
  .fill 0x53
  .fill 0x20
  .fill 0x53
  .fill 0x50
  .fill 0x41
  .fill 0x43
  .fill 0x45
  .fill 0x20
  .fill 0x54
  .fill 0x4F
  .fill 0x20
  .fill 0x53
  .fill 0x54
  .fill 0x41
  .fill 0x52
  .fill 0x54
  .fill 0x00