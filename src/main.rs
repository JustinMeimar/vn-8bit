// 16 8 bit registers named r[0] - r[f]
//

#![allow(unused)]
use clap::Parser;

use std::fs::File;
use std::fs;
use std::i64;
use std::str::FromStr;
use std::io::{Read, BufReader, self, prelude::*, BufRead, Error};

mod cpu;
mod prog;

use crate::cpu::CPU;
use crate::prog::Program;

// structs
#[derive(Parser)]
pub struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))] 
    path: std::path::PathBuf,

}

pub fn read_word(pc: u16, memory: [u8; 4096]) -> u16 {
    let idx: u16 = pc;
    ( (memory[idx as usize] as u16) << 8 | ( memory[idx as usize + 1] ) as u16 ) as u16
}         

fn main() {

   
    let addr = 0x20;
    let mut cpu = CPU::new_cpu();
        cpu.mem[127 as usize] = 0xAB; //171
        cpu.pc = addr;
    
    let args = Cli::parse(); 
    let mut program = Program::new();
        program.load_program( args.path );
    
        //cpu.execute_cycle();
}









