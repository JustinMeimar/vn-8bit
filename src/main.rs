// 16 8 bit registers named r[0] - r[f]

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

fn main() {
   
    let addr = 0x00;
    let mut cpu = CPU::new_cpu();
        cpu.pc = addr;
    
    let args = Cli::parse(); 
    let mut program = Program::new();
        program.load_program( args.path );
        

        // let mut i = 0;
        // while i < 20{
        //     println!("b1: {} b2: {} word:{}", program.memory[i as usize], program.memory[i as usize + 1 ], 
            
        //     (program.memory[i as usize] as u16) << 8 | (program.memory[i as usize + 1] as u16 ));
        //     i+=2; 
        // }

    cpu.execute_cycle(program.memory);
}









