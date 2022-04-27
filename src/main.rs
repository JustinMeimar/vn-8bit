// 16 8 bit registers named r[0] - r[f]
//

#![allow(unused)]
use clap::Parser;
use std::fs::File;

use std::io;
use std::io::Read;

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))] 
    path: std::path::PathBuf,

}

pub struct CPU {
    // 8-bit words 16-bit addresses 

    pub pc: u16,            //up to PC = 0x1000  valid range [0x0000, 0x1000]
    pub mem: [u8; 4096],    //
    pub reg: [u8; 16],      // 0 - F
    pub sp: u8,             // stack pointer
     
}

pub fn read_word(pc: u16, memory: [u8; 4096]) -> u16 {
    let idx: u16 = pc;
    ( (memory[idx as usize] as u16) << 8 | ( memory[idx as usize + 1] ) as u16 ) as u16
}

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
    
    pub fn execute_cycle(&mut self) {

        self.mem[self.pc as usize] = 0x03;
        self.mem[self.pc as usize + 1] = 0x40;
        let word: u16 = read_word(self.pc, self.mem);
        
        println!("{}", word);
        self.process_opcode(self.pc, word)
    }

    pub fn process_opcode(&mut self, pc: u16, word: u16){
        //mask the word
        let mut b0 = (word & 0xF000) >> 12;
        let mut b1 = (word & 0x0F00) >> 8;
        let mut b2 = (word & 0x00F0) >> 4;
        let mut b3 = (word & 0x000F); 
    
        match b0 {
            0x0 => {
                //impl lb
                println!("load byte");
                println!("b1: {} b2: {} b3: {}", b1, b2, b3);
                println!("{}", self.mem[b2 as usize]);
                self.reg[b2 as usize] = 127;
                let u = self.reg[b2 as usize];
                let fetch = self.mem[u as usize + b3 as usize];
                self.reg[b1 as usize] = fetch;
                println!("fetch: {}", fetch);    
            },
            0x1 => {
                //impl sb
                println!("{}", self.mem[b2 as usize + b3 as usize]);
                
                self.mem[b2 as usize + b3 as usize] = self.reg[b1 as usize]; 
            
                println!("{}", self.mem[b2 as usize + b3 as usize]);
            },
            0x2 => println!("and"),
            0x3 => println!("or"),
            0x4 => println!("xor"),
            /*
            0x5 => println!("2"),
            0x6 => println!("1"),
            0x7 => println!("2"),  
            0x8 => println!("1"),
            0x9 => println!("2"),
            0xA => println!("1"),
            0xB => println!("2"), 
            */ 
            _ => println!("else"),
        }
    }

}

fn main() {

    let args = Cli::parse(); 
    let addr = 0x20;
    let mut cpu = CPU::new_cpu();
        cpu.mem[127 as usize] = 0xAB; //171
        cpu.pc = addr;
        cpu.execute_cycle();
        
    println!("pc: {} mem[pc]: {}", cpu.pc, cpu.mem[cpu.pc as usize]);
    println!("{}",args.pattern); 

}









