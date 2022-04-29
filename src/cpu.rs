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
    pub mem: [u8; 4096],    //
    pub reg: [u8; 16],      // 0 - F
    pub sp: u8,             // stack pointer
 
}

// public functions 
pub fn read_word(pc: u16, memory: [u8; 4096]) -> u16 {
    let idx: u16 = pc;
    ( (memory[idx as usize] as u16) << 8 | ( memory[idx as usize + 1] ) as u16 ) as u16
}         

//impl
impl CPU {

    pub fn new_cpu() -> CPU {
        let new_cpu = CPU{
            pc: 0,
            mem: [0; 4096],
            reg: [0; 16],
            sp: 0,
        };
        new_cpu
    }
    
    pub fn execute_cycle(&mut self, prg_mem: [u8; 4096] ) {
        

        let word: u16 = read_word(self.pc, prg_mem);
        
        println!("word: {}", word);
        let res = self.process_opcode(self.pc, word);
        self.pc += 2;
        if res == Ok(true) {
            self.execute_cycle(prg_mem);
            self.print_reg_file();
        }else{
            println!("Program Terminated");
        }
        
    }

    pub fn process_opcode(&mut self, pc: u16, word: u16) -> Result<bool, bool>{
        //mask the word
        let mut b0: u8 = ((word & 0xF000) >> 12 as u8).try_into().unwrap();
        let mut b1: u8 = ((word & 0x0F00) >> 8 as u8).try_into().unwrap();
        let mut b2: u8 = ((word & 0x00F0) >> 4 as u8).try_into().unwrap();
        let mut b3: u8 = ((word & 0x000F) as u8).try_into().unwrap(); 
    
        match b0 {
            0xF => {
                //terminate program 
                Err(false) 
            },
            0x0 => {
                //impl lb  
                let r_up = self.reg[b2 as usize];
                let r_dwn = self.reg[b3 as usize];  
                let addr = ((r_up as u16) << 8 | r_dwn as u16) as usize;                   
    
                self.reg[b1 as usize] = self.mem[addr];
             
                Ok(true)
                
            },
            0x1 => {
                //impl sb

                self.mem[b2 as usize + b3 as usize] = self.reg[b1 as usize];  

                Ok(true)
            },
            0x2 => {println!("and"); Ok(true)},
            0x3 => {println!("or");  Ok(true)},
            0x4 => {println!("xor"); Ok(true)}, 
            0x5 => {println!("nor"); Ok(true)},
            0x6 => {println!("add"); Ok(true)},
            0x7 => {
                // impl addi  op $r1 |-- value --| range(0--127)
                println!("---------------------------------------");
                let sum = self.reg[b2 as usize] + self.reg[b3 as usize];
                self.reg[b1 as usize] = sum;
                
                Ok(true)
            },
            0x8 => {
                // impl add
                println!("---------------------------------------"); 
                let sum = b3 as u8 + self.reg[b2 as usize];
                self.reg[b1 as usize] = sum;

                Ok(true)
            },
            0x9 => {
                println!("---------------------------------------");
                
                let dest = ((b1 as u16) << 8 | (b2 as u16) << 4 | b1 as u16) as u16;
                self.pc = dest;
                Ok(true)
            },
            /*  
            0x8 => println!("1"),
            0x9 => println!("2"),
            0xA => println!("1"),
            0xB => println!("2"), 
            */ 
            _ => {println!("else"); Ok(true)},
        }
    }
    
    pub fn print_reg_file(&mut self){
        
        let mut chars: String = "".to_string();
        println!("\n-------------------------------");
        println!("0 1 2 3 4 5 6 7 8 9 A B C D E F");
        
        for r in 0..15{
            let mut reg = self.reg[r].to_string();
            chars.push_str(&reg);
            chars.push_str(" ");
            //self.reg[r]  
        }
        println!("{}", chars);
        println!("-------------------------------\n");
    } 

}

