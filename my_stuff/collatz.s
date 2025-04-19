	movi r3 main
	jalr r0  r3 

# Data Section:
line_index.2:
	.fill 0
cursor_index.1:
	.fill 0
RESOLUTION_REG:
	.fill 65532
FRAMEBUFFER_START:
	.fill 57344
ZERO_CHAR:
	.fill 48

# Code Section:
putchar:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -25
	sw r3  r2  -1
	movi r3 64
	sw r3  r2  -2
	lw r3  r2  -2
	movi r4 line_index.2
	lw r4  r4  0
	call umul
	sw r3  r2  -3
	movi r3 FRAMEBUFFER_START
	lw r3  r3  0
	lw r4  r2  -3
	add r3  r3  r4 
	sw r3  r2  -4
	movi r3 2
	sw r3  r2  -5
	movi r3 cursor_index.1
	lw r3  r3  0
	lw r4  r2  -5
	call udiv
	sw r3  r2  -6
	lw r3  r2  -4
	lw r4  r2  -6
	add r3  r3  r4 
	sw r3  r2  -7
	lw r3  r2  -7
	sw r3  r2  -8
	lw r3  r2  -8
	sw r3  r2  -9
	movi r3 1
	sw r3  r2  -10
	movi r3 10
	sw r3  r2  -11
	lw r3  r2  -1
	lw r4  r2  -11
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 putchar.end.9
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -10
putchar.end.9:
	lw r3  r2  -10
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 putchar.end.13
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -12
	lw r3  r2  -12
	movi r4 cursor_index.1
	sw r3  r4  0
	movi r3 line_index.2
	lw r3  r3  0
	sw r3  r2  -13
	movi r3 line_index.2
	lw r3  r3  0
	movi r4 1
	add r3  r3  r4 
	movi r4 line_index.2
	sw r3  r4  0
	movi r3 0
	sw r3  r2  -14
	lw r3  r2  -14
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
putchar.end.13:
	movi r3 1
	sw r3  r2  -15
	movi r3 cursor_index.1
	lw r3  r3  0
	lw r4  r2  -15
	and r3  r3  r4 
	sw r3  r2  -16
	lw r3  r2  -16
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 putchar.else.20
	jalr r0  r3 
	lw r3  r2  -9
	lw r4  r3  0
	sw r4  r2  -17
	movi r3 8
	sw r3  r2  -18
	lw r3  r2  -1
	lw r4  r2  -18
	call left_shift
	sw r3  r2  -19
	lw r3  r2  -17
	lw r4  r2  -19
	or r3  r3  r4 
	sw r3  r2  -20
	lw r3  r2  -9
	lw r4  r2  -20
	sw r4  r3  0
	movi r3 putchar.end.21
	jalr r0  r3 
putchar.else.20:
	lw r3  r2  -9
	lw r4  r2  -1
	sw r4  r3  0
putchar.end.21:
	movi r3 cursor_index.1
	lw r3  r3  0
	sw r3  r2  -21
	movi r3 cursor_index.1
	lw r3  r3  0
	movi r4 1
	add r3  r3  r4 
	movi r4 cursor_index.1
	sw r3  r4  0
	movi r3 1
	sw r3  r2  -22
	movi r3 40
	sw r3  r2  -23
	movi r3 cursor_index.1
	lw r3  r3  0
	lw r4  r2  -23
	cmp r3  r4 
	bae 1
	jmp 3
	movi r3 putchar.end.25
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -22
putchar.end.25:
	lw r3  r2  -22
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 putchar.end.28
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -24
	lw r3  r2  -24
	movi r4 cursor_index.1
	sw r3  r4  0
	movi r3 line_index.2
	lw r3  r3  0
	sw r3  r2  -25
	movi r3 line_index.2
	lw r3  r3  0
	movi r4 1
	add r3  r3  r4 
	movi r4 line_index.2
	sw r3  r4  0
putchar.end.28:
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
print_unisgned:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -11
	sw r3  r2  -1
	movi r3 10
	sw r3  r2  -2
	lw r3  r2  -1
	lw r4  r2  -2
	call umod
	sw r3  r2  -3
	lw r3  r2  -3
	sw r3  r2  -4
	movi r3 10
	sw r3  r2  -5
	lw r3  r2  -1
	lw r4  r2  -5
	call udiv
	sw r3  r2  -6
	lw r3  r2  -6
	sw r3  r2  -1
	movi r3 1
	sw r3  r2  -7
	movi r3 0
	sw r3  r2  -8
	lw r3  r2  -1
	lw r4  r2  -8
	cmp r3  r4 
	bnz 1
	jmp 3
	movi r3 print_unisgned.end.6
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -7
print_unisgned.end.6:
	lw r3  r2  -7
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 print_unisgned.end.8
	jalr r0  r3 
	lw r3  r2  -1
	call print_unisgned
	sw r3  r2  -9
print_unisgned.end.8:
	movi r3 ZERO_CHAR
	lw r3  r3  0
	lw r4  r2  -4
	add r3  r3  r4 
	sw r3  r2  -10
	lw r3  r2  -10
	call putchar
	sw r3  r2  -11
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
collatz:
	# Function Prologue
	sw r7  r1  -1
	sw r2  r1  -2
	addi r1  r1  -2
	addi r2  r1  0
	# Function Body
	addi r1  r1  -9
	sw r3  r2  -1
	movi r3 1
	sw r3  r2  -2
	lw r3  r2  -1
	lw r4  r2  -2
	and r3  r3  r4 
	sw r3  r2  -3
	lw r3  r2  -3
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 collatz.else.6
	jalr r0  r3 
	lw r3  r2  -1
	lw r4  r2  -1
	add r3  r3  r4 
	sw r3  r2  -4
	lw r3  r2  -4
	lw r4  r2  -1
	add r3  r3  r4 
	sw r3  r2  -5
	movi r3 1
	sw r3  r2  -6
	lw r3  r2  -5
	lw r4  r2  -6
	add r3  r3  r4 
	sw r3  r2  -7
	lw r3  r2  -7
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
	movi r3 collatz.end.9
	jalr r0  r3 
collatz.else.6:
	movi r3 1
	sw r3  r2  -8
	lw r3  r2  -1
	lw r4  r2  -8
	call right_shift
	sw r3  r2  -9
	lw r3  r2  -9
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
collatz.end.9:
	movi r3 0
	# Function Epilogue
	mov r1  r2 
	lw r7  r2  1
	lw r2  r2  0
	addi r1  r1  2
	jalr r0  r7 
main:
	movi r1 40959
	movi r2 40959
	addi r1  r1  -51
	call write_tilemap
	sw r3  r2  -1
	movi r3 RESOLUTION_REG
	lw r3  r3  0
	sw r3  r2  -2
	lw r3  r2  -2
	sw r3  r2  -3
	movi r3 1
	sw r3  r2  -4
	lw r3  r2  -3
	lw r4  r2  -4
	sw r4  r3  0
	movi r3 121
	sw r3  r2  -5
	lw r3  r2  -5
	sw r3  r2  -6
	lw r3  r2  -6
	sw r3  r2  -7
	movi r3 0
	sw r3  r2  -8
	lw r3  r2  -8
	sw r3  r2  -9
	movi r3 67
	sw r3  r2  -10
	lw r3  r2  -10
	call putchar
	sw r3  r2  -11
	movi r3 111
	sw r3  r2  -12
	lw r3  r2  -12
	call putchar
	sw r3  r2  -13
	movi r3 108
	sw r3  r2  -14
	lw r3  r2  -14
	call putchar
	sw r3  r2  -15
	movi r3 108
	sw r3  r2  -16
	lw r3  r2  -16
	call putchar
	sw r3  r2  -17
	movi r3 97
	sw r3  r2  -18
	lw r3  r2  -18
	call putchar
	sw r3  r2  -19
	movi r3 116
	sw r3  r2  -20
	lw r3  r2  -20
	call putchar
	sw r3  r2  -21
	movi r3 122
	sw r3  r2  -22
	lw r3  r2  -22
	call putchar
	sw r3  r2  -23
	movi r3 58
	sw r3  r2  -24
	lw r3  r2  -24
	call putchar
	sw r3  r2  -25
	movi r3 10
	sw r3  r2  -26
	lw r3  r2  -26
	call putchar
	sw r3  r2  -27
main.while.0.continue:
	movi r3 1
	sw r3  r2  -28
	movi r3 1
	sw r3  r2  -29
	lw r3  r2  -6
	lw r4  r2  -29
	cmp r3  r4 
	bnz 1
	jmp 3
	movi r3 main.end.33
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -28
main.end.33:
	lw r3  r2  -28
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 main.while.0.break
	jalr r0  r3 
	lw r3  r2  -6
	call print_unisgned
	sw r3  r2  -30
	movi r3 44
	sw r3  r2  -31
	lw r3  r2  -31
	call putchar
	sw r3  r2  -32
	lw r3  r2  -6
	call collatz
	sw r3  r2  -33
	lw r3  r2  -33
	sw r3  r2  -6
	movi r3 1
	sw r3  r2  -34
	lw r3  r2  -6
	lw r4  r2  -7
	cmp r3  r4 
	ba 1
	jmp 3
	movi r3 main.end.28
	jalr r0  r3 
	movi r3 0
	sw r3  r2  -34
main.end.28:
	lw r3  r2  -34
	movi r4 0
	cmp r3  r4 
	bz 1
	jmp 3
	movi r3 main.end.29
	jalr r0  r3 
	lw r3  r2  -6
	sw r3  r2  -7
main.end.29:
	movi r3 1
	sw r3  r2  -35
	lw r3  r2  -9
	lw r4  r2  -35
	add r3  r3  r4 
	sw r3  r2  -9
	movi r3 main.while.0.continue
	jalr r0  r3 
main.while.0.break:
	lw r3  r2  -6
	call print_unisgned
	sw r3  r2  -36
	movi r3 10
	sw r3  r2  -37
	lw r3  r2  -37
	call putchar
	sw r3  r2  -38
	movi r3 10
	sw r3  r2  -39
	lw r3  r2  -39
	call putchar
	sw r3  r2  -40
	movi r3 77
	sw r3  r2  -41
	lw r3  r2  -41
	call putchar
	sw r3  r2  -42
	movi r3 97
	sw r3  r2  -43
	lw r3  r2  -43
	call putchar
	sw r3  r2  -44
	movi r3 120
	sw r3  r2  -45
	lw r3  r2  -45
	call putchar
	sw r3  r2  -46
	movi r3 58
	sw r3  r2  -47
	lw r3  r2  -47
	call putchar
	sw r3  r2  -48
	movi r3 32
	sw r3  r2  -49
	lw r3  r2  -49
	call putchar
	sw r3  r2  -50
	lw r3  r2  -7
	call print_unisgned
	sw r3  r2  -51
	movi r3 0
	# Function Epilogue
	sys EXIT
