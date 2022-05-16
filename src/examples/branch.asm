main:
	addi $t1, $zero, 5
	addi $t2, $t2, 1
	beq $t1, $t2, exit_main
	jmp main	
exit_main:
	end


/// demostrates the use of beq