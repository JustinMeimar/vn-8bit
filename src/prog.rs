// dependancies
use std::fs::File;
use std::fs;
use std::i64;
use std::str::FromStr;
use std::io::{Read, BufReader, self, prelude::*, BufRead, Error, Write};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

// struct
pub struct Program {
    pub file_name: &'static str, 
    pub memory: [u8; 65535], // MAX program size is 4096 lines     
}

lazy_static! {
    //register alias mapping
    static ref HASHMAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("zero", "0");
        map.insert("t0", "1");
        map.insert("t1", "2");
        map.insert("t2", "3");
        map.insert("t3", "4");
        map.insert("s0", "5");
        map.insert("s1", "6");
        map.insert("s2", "7");
        map.insert("s3", "8");
        map.insert("s4", "9");
        map.insert("a0", "A");
        map.insert("a1", "B");
        map.insert("a2", "C");
        map.insert("v0", "D");
        map.insert("v1", "E");
        map.insert("ra", "F");
        map
    };
}

// impl
impl Program {
    //static fields

    pub fn new() -> Program {
        //init new program with memory 
        let new_program = Program{
            file_name: "",
            memory: [0; 65535],
        };
        new_program
    } 
    
    pub fn disassemble(&mut self, path: &std::path::PathBuf) -> io::Result<()>{
        // open file, scan for label: 
        // add {label: pc+2} to address lookup table
        // another traversal. remove label: 
        // replace label (no ":") with map["label"] --> pc+2
        // possibly remove indentation
        
        let mut src = File::open(&path)?;
        let mut data = String::new();
        let mut alias_table: HashMap<String, String> = HashMap::new();

        src.read_to_string(&mut data);
       
        let mut string: String = "".to_string();
        let mut pc_mirror: u32 = 0;
        for byte in data.chars(){
            if byte != '\n'{
                string.push_str(&byte.to_string());; 
            }
            if byte == '\n' { 
                println!("{}", string);
                self.build_alias_table(&string, pc_mirror, &mut alias_table);
                //println!("{}", new_str);
                string = "".to_string();
                pc_mirror += 2;
            }
        } 
        self.reassemble(path, &mut alias_table); 

        Ok(())    
    }
    
    pub fn reassemble(&mut self, path: &std::path::PathBuf, alias_table: &mut HashMap<String, String>) -> io::Result<()> {
       
        let f_name = path.file_name().unwrap().to_str().unwrap(); 
        let w_path = format!("/bin/{}", f_name);
        let mut w_file = File::create(&w_path).expect("Error creating file");
        let mut r_file = File::open(&path)?;
        let mut data = String::new();
        let mut t_string: String = "".to_string();
        r_file.read_to_string(&mut data); 
        
        for byte in data.chars(){
            
            if byte != '\n' && byte != ' ' {
                t_string.push_str(&byte.to_string());
                
            }else{
                t_string = t_string.split_whitespace().collect();  
                if alias_table.contains_key(&t_string) {
                    println!("swap {} with {}", t_string, alias_table.get(&t_string).unwrap());
                    t_string = alias_table.get(&t_string).unwrap().to_string();
                }
                println!("T_STRING: {}", t_string);
                //fs::write(w_file, t_string).expect("invaldi write");
                t_string = "".to_string(); 
            }
        }
           
        Ok(())
    }
    
    pub fn build_alias_table(&mut self, string: &String, pc_mirror: u32, map: &mut HashMap<String, String>){
      
        // money 
        println!("before: {}", string); 
        let mut chars = "".to_string();
        let mut v: Vec<&str> = string.split(" ").collect(); 
        if v[0].ends_with(":"){ 
            chars.push_str(&pc_mirror.to_string());    
            map.insert(v[0].replace(":", ""), pc_mirror.to_string());
            map.insert(v[0].to_string(), "nop".to_string());
        }else{
            chars.push_str(string);
        }     
        chars.push_str("\n");
        println!("after: {}", chars);    

    }

    pub fn load_program(&mut self, path: &std::path::PathBuf) -> io::Result<()> {   
        // read lines from asm program, writes the binary to program memory  
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut i: u32 = 0; 
        
        println!(".asm interpreted as:\n------------------");
        for line in reader.lines() {    
            let cpy = line?.clone();
            let instr = self.parse_instr(&cpy, i); 
            if instr.unwrap() == "lb" || instr.unwrap() == "sb" {
                i+=2; //skip pc by two
            }
            i += 2;            
        }
        Ok(())  
    }
    
    pub fn store_word_from_bytes(&mut self, byte1: u8, byte2: u8, idx: u32){
        self.memory[idx as usize]     = byte1;
        self.memory[idx as usize + 1] = byte2;
    }
    
    pub fn parse_addr(&mut self, bits: &str) -> u16{
        let addr: &str = &bits.replace("0x", ""); 
        let a64: i64 = i64::from_str_radix(addr, 16).unwrap(); 
        let a16: u16 = a64 as u16;
       
        a16
    }
    
    pub fn parse_register(&mut self, reg: &str, base: u32) -> u8{ 

        let mut r:  &str = &reg.replace("$", "").replace(",", "").to_string();
        let mapped = HASHMAP.get(&r);
        
        if mapped != None {
            //println!("match: {}", mapped.unwrap());
            r = mapped.unwrap();
        }
        let r64: i64 = i64::from_str_radix(r, base).unwrap(); 
        let r8:   u8 = r64 as u8;      
         
        r8
    }
    
    pub fn parse_instr(&mut self, string : &str, mut i: u32) -> Result<&str, bool>{
        // separate into components op $r1, $r2, <imm> ..  
        let v: Vec<&str> = string.split(' ').collect();
        let op: &str = v[0]; 
        let mut word: u16;
        
        // match the opcode to the instruction per ISA schema
        match op {
            "end" => {
                // end program sential 
                let op = 0xFF;
                self.memory[i as usize]    = 0xFF;
                self.memory[i as usize +1] = 0xFF;

                Ok("end")
            },
            "lb"  => {
                let op = 0x10;
                let byte1 = op | self.parse_register(&v[1], 16);
                let addr = self.parse_addr(v[2]);
                let byte2 = ((addr & 0xFF00) >> 8) as u8;
                let byte3 = ((addr & 0x00FF) as u8);
                
                self.memory[i as usize] = byte1;
                self.memory[i as usize +1] = byte2;
                self.memory[i as usize +2] = byte3;
                self.memory[i as usize +3] = 0x00;

                i+=2;
                self.print_instr(i, &v[0], &v[1], Some(&v[2]), None);

                Ok("lb")
            },
            "sb"  => {
                let op = 0x20;

                let byte1 = op | self.parse_register(&v[1], 16); 
                let addr = self.parse_addr(v[2]);  
                let byte2 = ((addr & 0xFF00) >> 8) as u8;
                let byte3 = ((addr & 0x00FF) as u8);

                self.memory[i as usize] = byte1;
                self.memory[i as usize +1] = byte2;    
                self.memory[i as usize +2] = byte3;
                self.memory[i as usize +3] = 0x00;

                i+=2;

                self.print_instr(i, &v[0], &v[1], Some(&v[2]), None);

                Ok("sb")
            },
            "and" => {
                let op = 0x30;
                let r1 = self.parse_register(&v[1], 16);
                let r2 = self.parse_register(&v[2], 16);
                let r3 = self.parse_register(&v[3], 16);  

                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | r3;

                self.store_word_from_bytes(byte1, byte2, i);

                Ok("and")
            },
            "or"  => {
                let op = 0x40;
                let r1 = self.parse_register(&v[1], 16);
                let r2 = self.parse_register(&v[2], 16);
                let r3 = self.parse_register(&v[3], 16);  

                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | r3;

                self.store_word_from_bytes(byte1, byte2, i);

                Ok("or")
            },
            "nor" => {
                let op = 0x50;
                let r1 = self.parse_register(&v[1], 16);
                let r2 = self.parse_register(&v[2], 16);
                let r3 = self.parse_register(&v[3], 16);  

                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | r3;

                self.store_word_from_bytes(byte1, byte2, i);

                Ok("not")
            },
            "xor" => {
                let op = 0x60;
                let r1 = self.parse_register(&v[1], 16);
                let r2 = self.parse_register(&v[2], 16);
                let r3 = self.parse_register(&v[3], 16);  

                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | r3;

                self.store_word_from_bytes(byte1, byte2, i);

                Ok("xor")
            },
            "add" => {
                // store binary format for add 
                let op = 0x70; 
                let r1 = self.parse_register(&v[1], 16); 
                let r2 = self.parse_register(&v[2], 16); 
                let r3 = self.parse_register(&v[3], 16); 
                
                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | r3;
                
                self.store_word_from_bytes(byte1, byte2, i);
                
                Ok("add")
           }, 
           "addi" => {
               // store binary format for addi 
                let op = 0x80;
                let r1 = self.parse_register(&v[1], 16); //r1
                let r2 = self.parse_register(&v[2], 16); //r2
                let im = self.parse_register(&v[3], 10); //immediate

                let byte1: u8 = op | r1;
                let byte2: u8 = (r2 << 4) | im;

                self.store_word_from_bytes(byte1, byte2, i);
                self.print_instr(i, &v[0], &v[1], Some(&v[2]), Some(&v[3]));

                Ok("addi")
            },
            "jmp" => {
                //jmp to specific address (no aliasing yet)
                let op = 0x90;
                let addr = self.parse_addr(v[1]);
                let byte1: u8 = op | ((addr & 0x0F00) >> 8) as u8;
                let byte2: u8 = (addr & 0x00FF) as u8;
                
                self.store_word_from_bytes(byte1, byte2, i);
                self.print_instr(i, &v[0], &v[1], None, None); 
                
                Ok("jmp")
            },
            "beq" => {
                
                let op = 0xA0;
                let r1 = self.parse_register(&v[1], 16); 
                let r2 = self.parse_register(&v[2], 16);
                let im = self.parse_register(&v[3], 10); //range -8 <-> + 7
                
                let byte1: u8 = (op | (r1));
                let byte2: u8 = (r2<<4 | im & 0x0F); 

                self.store_word_from_bytes(byte1, byte2, i);
                self.print_instr(i, &v[0], &v[1], Some(&v[2]), Some(&v[3]));     
                
                Ok("beq")
            },
            // Default case
            _ => {
                println!("Invalid Instruction (Parsed)");
                Ok("invalid") 
            }
        } 
    }

    pub fn print_instr(&mut self, line: u32, op: &str, b2: &str, b1: Option<&str>, b0: Option<&str>){         
        let mut chars: String = "".to_string();
        chars.push_str("0x----  "); 
        chars.push_str(op); // there will always be an opcode
        chars.push_str(" "); 
        chars.push_str(b2); // there will always be at least 1 arg
        match b1 {
            Some(b1) => {
                chars.push_str(" ");
                chars.push_str(b1);
                chars.push_str(" ");
            },
            _ => {}
        }
        match b0 {
            Some(b0) => chars.push_str(b0),
            _ => {}
        }
        println!("{}", chars);
    }
}
