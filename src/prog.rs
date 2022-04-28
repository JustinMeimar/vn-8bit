// dependancies
use std::fs::File;
use std::fs;
use std::i64;
use std::str::FromStr;
use std::io::{Read, BufReader, self, prelude::*, BufRead, Error};
// struct

pub struct Program {
    pub file_name: &'static str, 
    pub memory: [u8; 4096], // MAX program size is 4096 lines 
}

// impl

impl Program {

    pub fn new() -> Program {
        
        let new_program = Program{
            file_name: "",
            memory: [0; 4096],
        };
        new_program
    } 

    pub fn load_program(&mut self, path: std::path::PathBuf) -> io::Result<()> {   
        // read lines from asm program 
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut i: u32 = 0; 
        for line in reader.lines() {
            
            let cpy = line?.clone();
            self.parse_instr(&cpy, i);
            i += 1;
            
        }

        Ok(())
            
    }

    pub fn parse_instr(&mut self, string : &str, i: u32){
        // separate into components op $r1, $r2, <imm> ..  
        let v: Vec<&str> = string.split(' ').collect();
        let op: &str = v[0];
       
        let mut word: u16;
        match op {
            "addi"  => {
                
                let op = 0x80;
                let r1 = self.parse_register(&v[1], 16); //r1
                let r2 = self.parse_register(&v[2], 16); //r2
                let im = self.parse_register(&v[3], 10); //immediate
                
                let byte1: u8 = op | r1;
                let byte2: u8 = r2 | im;

                self.memory[i as usize]     = byte1;
                self.memory[i as usize + 1] = byte2;
                
                println!("{}{}", self.memory[i as usize], self.memory[i as usize +1 ]);
            },
            "add"   =>{
               
            }
            _       => println!("default")
        }
        
    }
    
    pub fn parse_register(&mut self, reg: &str, base: u32) -> u8{
        
        let r:  &str = &reg.replace("$", "").replace(",", "").to_string();
        let r64: i64 = i64::from_str_radix(r, base).unwrap();  
        let r8:   u8 = r64 as u8; 
            
        r8
    } 

    pub fn encode_r_type(&mut self, comps: Vec<&str>, i: u32){
        // store r-type instruction into program memory 
        let op = comps[0];
        let r1 = comps[1];
        let imm = comps[2];
     
    }

}
