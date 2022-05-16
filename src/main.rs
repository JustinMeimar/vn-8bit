// 16 8 bit registers named r[0] - r[f]
#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use clap::Parser;
use std::fs::File;
use std::fs;
use std::i64;
use std::str::FromStr;
use std::io::{Read, BufReader, self, prelude::*, BufRead, Error};



mod cpu;
//mod prog;
mod assembler;

use crate::cpu::CPU;
//use crate::prog::Program;
use crate::assembler::Program;
// structs
#[derive(Parser)]
pub struct Cli {
    #[clap(parse(from_os_str))] 
    path: std::path::PathBuf,
}

fn main() {
   
    let addr = 0x00;
    let mut cpu = CPU::new_cpu();
        cpu.pc = addr;
    
    let args = Cli::parse(); 
    let mut program = Program::new(); 
        program.load_program( &args.path );
        
    let mut res: bool = true; 
    println!("\nWhat the Reigster File Looks Like: ");
    //cpu.print_binary(program.memory);
    while res == true{
        res = cpu.execute_cycle(program.memory).unwrap(); 
    }
    
}









