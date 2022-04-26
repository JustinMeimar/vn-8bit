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
    
    pub pc: u16,            //up to PC = 0x1000  valid range [0x0000, 0x1000]
    pub mem: [u8; 4096],    //
    pub reg: [u8; 16],      // 0 - F
    pub sp: u8,             // stack pointer
 
}

pub fn read_word(pc: u16, memory: [u8; 4096]) -> u16 {
    let idx: u16 = pc;
    ( (memory[idx as usize] as u16) << 8 | ( memory[idx as usize] ) as u16 ) as u16
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
    
    pub fn execute_cylce(&self) {
        
        let word: u16 = read_word(self.pc, self.mem);
        self.process_word(word);
    
    }

    pub fn process_word(&mut self, )


    /* 
    pub fn load_memory(&self, path: std::path::PathBuf) -> io::Result<()>{
        
        let f = File::open(path)?;
        let mut reader = io::BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        for value in buffer {
            println!("value: {}", value);
        }
        
        let u: &[4096] = &buffer;
        Ok(())
    }   */
    /* 
    pub fn fetch_instr(&self, pc: u16) -> u16 {
        let i: u16 = pc; 
        
        self.pc = self.pc + 2;

        (self.mem[i as usize] as u16) << 8 | self.mem[i as usize +1] as u16 
        
    }*/

}

fn main() {
    println!("Hello World");
    let args = Cli::parse();
    let cpu = CPU::new_cpu(); 
        //cpu.fetch_instr(100);
        //cpu.load_memory(args.path); 
        cpu.execute_cycle(); 
    
    println!("{}",args.pattern); 

}









