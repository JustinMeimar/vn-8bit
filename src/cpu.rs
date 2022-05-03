//nothing to see here
use std::fs::File;
use std::fs;
use std::i64;
use std::str::FromStr;
use std::io::{Read, BufReader, self, prelude::*, BufRead, Error};


//struct
pub struct CPU {
    // 8-bit words 16-bit addresses 

    pub pc: u16,            //up to PC = 0x1000  valid range [0x0000, 0x1000]
    pub mem: [u8; 65535],    //
    pub reg: [u8; 16],      // 0 - F
    // special registers
    pub sp: u8,             // stack pointer
    pub ra_hi: u8,    //intermediate for load/store
    pub ra_lo: u8,    //intermediate for load/store
}

// public functions 
pub fn read_word(pc: u16, memory: [u8; 65535]) -> u16 {
    let idx: u16 = pc;
    ( (memory[idx as usize] as u16) << 8 | ( memory[idx as usize + 1] ) as u16 ) as u16
}         

//impl
impl CPU {

    pub fn new_cpu() -> CPU {
        let new_cpu = CPU{
            pc: 0,
            mem: [0; 65535],
            reg: [0; 16],
            sp: 0,
            ra_lo: 0,
            ra_hi: 0,
        };
        new_cpu
    }
    
    pub fn execute_cycle(&mut self, prg_mem: [u8; 65535] ) -> Result<bool, bool> {
        
        let word: u16 = read_word(self.pc, prg_mem);
        let res = self.process_opcode(self.pc, word, prg_mem);
        if res != Ok(true){
            println!("Program terminated.... "); 
            Ok(false)
 
        }else{
            self.print_reg_file();
            self.pc += 2; 
          
            Ok(true)
        } 
    }

    pub fn process_opcode(&mut self, pc: u16, word: u16, prg_mem: [u8; 65535]) -> Result<bool, bool>{
        //mask the word 
        let mut b0: u8 = ((word & 0xF000) >> 12 as u8).try_into().unwrap();
        let mut b1: u8 = ((word & 0x0F00) >> 8 as u8).try_into().unwrap();
        let mut b2: u8 = ((word & 0x00F0) >> 4 as u8).try_into().unwrap();
        let mut b3: u8 = ((word & 0x000F) as u8).try_into().unwrap(); 
        
        match b0 {
            0x0 => {
                //impl nop
                Ok(true)
            }, 
            0x1 => {
                //impl lb  
                let address: u16 = read_word(self.pc + 2, prg_mem); 
                self.reg[b1 as usize] = self.mem[address as usize];
                self.pc += 2;
                Ok(true) 
            },
            0x2 => {
                //impl sb                
                let address: u16 = read_word(self.pc + 2, prg_mem); 
                self.mem[address as usize] = self.reg[b1 as usize];
                self.pc += 2; 
                Ok(true)
            },
            0x3 => {
                //impl and
                self.reg[b1 as usize] = self.reg[b2 as usize] & self.reg[b3 as usize];
                Ok(true)
            },
            0x5 => {
                //impl or
                self.reg[b1 as usize] = self.reg[b2 as usize] | self.reg[b3 as usize];
                Ok(true)
            },
            0x5 => {
                //impl nor
                self.reg[b1 as usize] = !(self.reg[b2 as usize] | self.reg[b3 as usize]);
                Ok(true)
            },   
            0x6 => {
                //impl xor
                self.reg[b1 as usize] = self.reg[b2 as usize] ^ self.reg[b3 as usize]; 
                Ok(true)
            },
            0x7 => {
                // impl add op $r1 |-- value --| range(0--127)
                let sum = self.reg[b2 as usize] + self.reg[b3 as usize];
                self.reg[b1 as usize] = sum;
                
                Ok(true)
            },
            0x8 => {
                // impl addi 
                let sum = b3 as u8 + self.reg[b2 as usize];
                self.reg[b1 as usize] = sum;

                Ok(true)
            },
            0x9 => {
                // impl jmp
                let dest = ((b1 as u16) << 8 | (b2 as u16) << 4 | b3 as u16) as u16; 
                self.pc = dest;
                Ok(true)
            },
            0xA => {
                //impl beq

                if self.reg[b1 as usize] == self.reg[b2 as usize] {
                    // take branch 
                    let dif = b3 as i16;
                    if dif < 0 {
                        self.pc -= b3 as u16;
                    }else{
                        self.pc += b3 as u16;
                    } 
                }
                Ok(true)
            },
            0xB => {
                // impl jal
                let return_address: u16 = read_word(self.pc + 2, prg_mem);
                let address: u16 = (word & 0x0FFF);
                
                self.pc = address;
                self.ra_hi = ((return_address & 0xFF00) >> 8) as u8; 
                self.ra_lo = (return_address & 0x00FF) as u8;

                Ok(true)
            },
            0xC => {
                // impl jr 
                let address = ((self.ra_hi as u16) << 8) | self.ra_lo as u16;
                self.pc = address;              

                Ok(true)
            },
            0xF => {
                //terminate program 
                Ok(false) 
            },
            /*  
            0x8 => println!("1"),
            0x9 => println!("2"),
            0xA => println!("1"),
            0xB => println!("2"), 
            */ 
            _ => {println!("Invalid Instruction"); Err(false)},
        }
    }
    
    pub fn print_reg_file(&mut self){
        
        let mut chars: String = "".to_string();
        println!("-------------------------------");
        println!("PC: {}", self.pc);
        println!("$0 | $t0 | $t1 | $t2 | $t3 | $s0 | $s1 | $s2 | $s3 | $s4 | $a0 | $a1 | $a2 | $v0 | $v1 | $ra |");

        for r in 0..16{
            let mut reg = self.reg[r].to_string();
            chars.push_str(&reg);
            chars.push_str("  |  ");
            //self.reg[r]  
        }
        println!("{}", chars);
        
    } 
    pub fn print_binary(&mut self, prg_mem: [u8; 65535]){
        let mut chars: String = "".to_string(); 
        let mut i: u32 = 0; 
        for byte in prg_mem {
            chars.push_str(&byte.to_string());
            chars.push_str(&" ".to_string());
            i+=1;
            if i % 2 == 0 {
                println!("{}", chars);
                chars = "".to_string();
            } 
            if i > 109 {
                break;
            }
        }


    }
}

