# **Assembly Decoder and Simulator for 8-bit Processor**
## **Download and Build**

#### Requires Rust to be installed

## **Regiser Use**
|Register|Non-Aliased|Summary|
|---|---|---|
|t0 - t4|0x0 - 0x4|temporary registers|
|s0 - s4|0x5 - 0x9|saved registers|
|a0 - a1| 0xA-0xB| argument registers|
|ra|0xF|return address|

## **Instruction Formats**
|Format| Internal Representation| Notes|
|---|---|---|
|m-type| 1111 1111 1111 1111 <br /> 1111 1111 0000 0000| 31-28 --- Op Code <br  /> 27-24 --- Register <br /> 23-16 --- Address upper half <br /> 15-0 ----- Address lower half|
|r-type| 1111 1111 1111 1111| 15-12 --- Op Code <br /> 11-8 --- Register 1 <br /> 7-4 ----- Register 2 <br /> 3-0 ----- Register 3|
|i-type| 1111 1111 1111 1111| 15-12 --- Op Code <br /> 11-8 ---- Register 1 <br /> 7-4 ------ Register 2 <br /> 3-0 ------ Immediate
|j-type| 1111 1111 1111 1111| 15-12 ---- Op Code <br /> 11-0 ------ Address|
## **Instruction Set**

|Instruction|Op Code|Format|example|Notes|
|----|-----|------|-----|---|
|nop|0x0|no type| nop| none operator.
|lb|0x1|m-type| lb $v, addr| This instruction uses two clock cycles.
|sb|0x2|m-type| sb $v, addr| This instruction uses two clock cycles.
|and|0x3|r-type| and $v, $u, $t| Standard bitwise and operation
|or|0x4|r-type|or $v, $u, $t| standard bitwise or
|nor|0x5|r-type|nor $v, $u, $t| standard bitwise nor
|xor|0x6|r-type|xor $v, $u, $t| standard bitwise xor
|add|0x7|r-type|add $v, $u, $t| standard bitwise add
|addi|0x8|i-type|addi $v, $u, 255| add immidiate in range (-128 .. 127)
|jmp|0x9|j-type|jmp 0x000| jmp in range 0 -> 2^12
|beq|0xA|i-type|beq $v, $u 0xbb| branch from pc in range (-16..15)
|jal|0xB|j-type*|jal sbrt| jump to sub-routine and save PC in $ra
|jr|0xC|j-type*|jr $ra| return to address stored in $ra, 
||0xD|
||0xE|
|end|0xF| no type| end| terminates any program from anywhere
## **The Assember**
### Example
        
        // 0x00  addi $t0, $t0, 1
        // 0x02  loop:
        // 0x04     li $v0, $v0, 1
        // 0x06     beq $s0, $s0 endloop
        // 0x08       j loop
        // 0x0A
        // 0x0C  endloop:
        // 0x0E     addi $v0, 10
        // 0x10     

        // 0x00  addi $1, $1, 1
        // 0x02  nop
        // 0x04  li $D, $D, 1
        // 0x06  beq $5, $5, 0x0C
        // 0x08  j 0x04
        // 0x0A  nop
        // 0x0C  addi $D, 10
        
