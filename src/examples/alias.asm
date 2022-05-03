main:
loop:
	addi $s0, $0, 5
	addi $t0, $t0, 1
	beq $t0, $s0, exit_loop
	jmp loop
exit_loop:
	addi $v0, $v0, 1
end
