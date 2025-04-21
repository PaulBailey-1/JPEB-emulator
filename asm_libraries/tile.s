# Data Section:

# Code Section:
write_solid_tile:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -12
	sw r3  r2  -1
	sw r4  r2  -2
	movi r3 49152
	sw r3  r2  -3
	lw r3  r2  -3
	sw r3  r2  -4
	lw r3  r2  -4
	sw r3  r2  -5
	lw r3  r2  -5
	sw r3  r2  -6
	movi r3 64
	sw r3  r2  -7
	lw r3  r2  -7
	lw r4  r2  -1
	call umul
	sw r3  r2  -8
	lw r3  r2  -6
	lw r4  r2  -8
	add r3  r3  r4 
	sw r3  r2  -6
	movi r3 0
	sw r3  r2  -9
write_solid_tile.for.0.start:
	movi r3 1
	sw r3  r2  -10
	lw r3  r2  -9
	movi r4 64
	cmp r3  r4 
	bl 1
	jmp 3
	movi r3 write_solid_tile.end.7
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -10
write_solid_tile.end.7:
	lw r3  r2  -10
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 write_solid_tile.for.0.break
	jalr r0  r3 
	lw r3  r2  -9
	movi r4 1
	call smul
	sw r3  r2  -11
	lw r3  r2  -6
	lw r4  r2  -11
	add r3  r3  r4 
	sw r3  r2  -12
	lw r3  r2  -12
	lw r4  r2  -2
	sw r4  r3  0
write_solid_tile.for.0.continue:
	lw r3  r2  -9
	movi r4 1
	add r3  r3  r4 
	sw r3  r2  -9
	movi r3 write_solid_tile.for.0.start
	jalr r0  r3 
write_solid_tile.for.0.break:
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
draw_pixel:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -43
	sw r3  r2  -1
	sw r4  r2  -2
	sw r5  r2  -3
	movi r3 FRAMEBUFFER_START
	lw r3  r3  0
	sw r3  r2  -4
	lw r3  r2  -4
	sw r3  r2  -5
	movi r3 2
	sw r3  r2  -6
	lw r3  r2  -1
	lw r4  r2  -6
	call udiv
	sw r3  r2  -7
	lw r3  r2  -7
	movi r4 1
	call umul
	sw r3  r2  -8
	lw r3  r2  -5
	lw r4  r2  -8
	add r3  r3  r4 
	sw r3  r2  -9
	movi r3 64
	sw r3  r2  -10
	lw r3  r2  -10
	lw r4  r2  -2
	call umul
	sw r3  r2  -11
	lw r3  r2  -11
	movi r4 1
	call umul
	sw r3  r2  -12
	lw r3  r2  -9
	lw r4  r2  -12
	add r3  r3  r4 
	sw r3  r2  -13
	lw r3  r2  -13
	lw r4  r3  0
	sw r4  r2  -14
	lw r3  r2  -14
	sw r3  r2  -15
	movi r3 1
	sw r3  r2  -16
	lw r3  r2  -1
	lw r4  r2  -16
	and r3  r3  r4 
	sw r3  r2  -17
	lw r3  r2  -17
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 draw_pixel.else.25
	jalr r0  r3 
	movi r3 2
	sw r3  r2  -18
	lw r3  r2  -1
	lw r4  r2  -18
	call udiv
	sw r3  r2  -19
	lw r3  r2  -19
	movi r4 1
	call umul
	sw r3  r2  -20
	lw r3  r2  -5
	lw r4  r2  -20
	add r3  r3  r4 
	sw r3  r2  -21
	movi r3 64
	sw r3  r2  -22
	lw r3  r2  -22
	lw r4  r2  -2
	call umul
	sw r3  r2  -23
	lw r3  r2  -23
	movi r4 1
	call umul
	sw r3  r2  -24
	lw r3  r2  -21
	lw r4  r2  -24
	add r3  r3  r4 
	sw r3  r2  -25
	movi r3 8
	sw r3  r2  -26
	lw r3  r2  -3
	lw r4  r2  -26
	call left_shift
	sw r3  r2  -27
	movi r3 255
	sw r3  r2  -28
	lw r3  r2  -15
	lw r4  r2  -28
	and r3  r3  r4 
	sw r3  r2  -29
	lw r3  r2  -27
	lw r4  r2  -29
	or r3  r3  r4 
	sw r3  r2  -30
	lw r3  r2  -25
	lw r4  r2  -30
	sw r4  r3  0
	movi r3 draw_pixel.end.39
	jalr r0  r3 
draw_pixel.else.25:
	movi r3 2
	sw r3  r2  -31
	lw r3  r2  -1
	lw r4  r2  -31
	call udiv
	sw r3  r2  -32
	lw r3  r2  -32
	movi r4 1
	call umul
	sw r3  r2  -33
	lw r3  r2  -5
	lw r4  r2  -33
	add r3  r3  r4 
	sw r3  r2  -34
	movi r3 64
	sw r3  r2  -35
	lw r3  r2  -35
	lw r4  r2  -2
	call umul
	sw r3  r2  -36
	lw r3  r2  -36
	movi r4 1
	call umul
	sw r3  r2  -37
	lw r3  r2  -34
	lw r4  r2  -37
	add r3  r3  r4 
	sw r3  r2  -38
	movi r3 255
	sw r3  r2  -39
	lw r3  r2  -3
	lw r4  r2  -39
	and r3  r3  r4 
	sw r3  r2  -40
	movi r3 65280
	sw r3  r2  -41
	lw r3  r2  -15
	lw r4  r2  -41
	and r3  r3  r4 
	sw r3  r2  -42
	lw r3  r2  -40
	lw r4  r2  -42
	or r3  r3  r4 
	sw r3  r2  -43
	lw r3  r2  -38
	lw r4  r2  -43
	sw r4  r3  0
draw_pixel.end.39:
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
read_pixel:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -40
	sw r3  r2  -1
	sw r4  r2  -2
	movi r3 FRAMEBUFFER_START
	lw r3  r3  0
	sw r3  r2  -3
	lw r3  r2  -3
	sw r3  r2  -4
	movi r3 2
	sw r3  r2  -5
	lw r3  r2  -1
	lw r4  r2  -5
	call udiv
	sw r3  r2  -6
	lw r3  r2  -6
	movi r4 1
	call umul
	sw r3  r2  -7
	lw r3  r2  -4
	lw r4  r2  -7
	add r3  r3  r4 
	sw r3  r2  -8
	movi r3 64
	sw r3  r2  -9
	lw r3  r2  -9
	lw r4  r2  -2
	call umul
	sw r3  r2  -10
	lw r3  r2  -10
	movi r4 1
	call umul
	sw r3  r2  -11
	lw r3  r2  -8
	lw r4  r2  -11
	add r3  r3  r4 
	sw r3  r2  -12
	lw r3  r2  -12
	lw r4  r3  0
	sw r4  r2  -13
	lw r3  r2  -13
	sw r3  r2  -14
	movi r3 1
	sw r3  r2  -15
	lw r3  r2  -1
	lw r4  r2  -15
	and r3  r3  r4 
	sw r3  r2  -16
	lw r3  r2  -16
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 read_pixel.else.23
	jalr r0  r3 
	movi r3 2
	sw r3  r2  -17
	lw r3  r2  -1
	lw r4  r2  -17
	call udiv
	sw r3  r2  -18
	lw r3  r2  -18
	movi r4 1
	call umul
	sw r3  r2  -19
	lw r3  r2  -4
	lw r4  r2  -19
	add r3  r3  r4 
	sw r3  r2  -20
	movi r3 64
	sw r3  r2  -21
	lw r3  r2  -21
	lw r4  r2  -2
	call umul
	sw r3  r2  -22
	lw r3  r2  -22
	movi r4 1
	call umul
	sw r3  r2  -23
	lw r3  r2  -20
	lw r4  r2  -23
	add r3  r3  r4 
	sw r3  r2  -24
	lw r3  r2  -24
	lw r4  r3  0
	sw r4  r2  -25
	lw r3  r2  -25
	sw r3  r2  -26
	lw r3  r2  -26
	sw r3  r2  -27
	lw r3  r2  -27
	movi r4 8
	call right_shift
	sw r3  r2  -27
	lw r3  r2  -27
	sw r3  r2  -28
	lw r3  r2  -28
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
	movi r3 read_pixel.end.35
	jalr r0  r3 
read_pixel.else.23:
	movi r3 2
	sw r3  r2  -29
	lw r3  r2  -1
	lw r4  r2  -29
	call udiv
	sw r3  r2  -30
	lw r3  r2  -30
	movi r4 1
	call umul
	sw r3  r2  -31
	lw r3  r2  -4
	lw r4  r2  -31
	add r3  r3  r4 
	sw r3  r2  -32
	movi r3 64
	sw r3  r2  -33
	lw r3  r2  -33
	lw r4  r2  -2
	call umul
	sw r3  r2  -34
	lw r3  r2  -34
	movi r4 1
	call umul
	sw r3  r2  -35
	lw r3  r2  -32
	lw r4  r2  -35
	add r3  r3  r4 
	sw r3  r2  -36
	lw r3  r2  -36
	lw r4  r3  0
	sw r4  r2  -37
	lw r3  r2  -37
	sw r3  r2  -38
	lw r3  r2  -38
	sw r3  r2  -39
	lw r3  r2  -39
	movi r4 255
	and r3  r3  r4 
	sw r3  r2  -39
	lw r3  r2  -39
	sw r3  r2  -40
	lw r3  r2  -40
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
read_pixel.end.35:
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
