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

## **Instruction Set**

|Instruction|Op Code|Format|example|Notes|
|----|-----|------|-----|---|
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