main:
	addi $s0, $zero, 6
	add $t1, $zero, $zero
	loop:	
		jal sbrt
		beq $t0, $s0, end_loop
		jmp loop
	end_loop:
		addi $v0, $v0, 1
		end
sbrt:
	addi $t0, $t0, 1
	jr $ra

/// Demonstrates the use of "jump and link" and "jr"
/// Aswell as label aliasing
