main:
	addi $s0, $0, 5
	addi $t0, $0, 0	
	addi $t1, $0, 0
	loop:
		addi $t0, $t0, 1
		jal sbrt
		beq $t1, $s0, end_loop
	end_loop:
		addi $v0, $v0, 1
		end
sbrt:
	addi $t1, $t1, 1
	jr $ra
	end	
