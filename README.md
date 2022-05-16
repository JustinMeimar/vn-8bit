# **Assembler and Simulator for 8-bit Processor**
## **Download and Build**

#### Requires Rust to be installed
#### 
        Cargo run examples/<example.asm>

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
#### Input code written in assembly (with the Instruction Set referenced above)and see it transformed into machine code our virtual CPU can actually process. The main technical achievments being address aliasing and register alias  mapping. For example:
|pre assembled code|assembled code|
|----|----|
|beq $s0, $v0, endloop|beq $5, $D, 10|

#### The Assembled code format is easy to parse and execute.

### Examples
        
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
        
        

