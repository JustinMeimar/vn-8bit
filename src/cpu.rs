//nothing to see here


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
    
    pub fn execute_cycle(&mut self) {
        

        self.mem[self.pc as usize] = 0x71;
        self.mem[self.pc as usize + 1] = 0xFF;

        let word: u16 = read_word(self.pc, self.mem);
        
        println!("instr: {}", word);

        //explicitly set the value at memory to load from to 125
        self.mem[0x02AA as usize] = 125; 
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
                
                // read 16 bit address from 2 8-bit registers 
                self.reg[b2 as usize] = 0x02;
                self.reg[b3 as usize] = 0xAA;
                
                
                let r_up = self.reg[b2 as usize];
                let r_dwn = self.reg[b3 as usize];
                
                // address = r_up << 8 | r_dwn
                let addr = ((r_up as u16) << 8 | r_dwn as u16) as usize;                
             
                println!("mem[addr]: {}", self.mem[addr]);
                println!("fetch: {}", self.reg[b1 as usize]);
                               
                self.reg[b1 as usize] = self.mem[addr];

                println!("fetch: {}", self.reg[b1 as usize]);   
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

            
            0x5 => println!("nor"),
            0x6 => println!("add"),
            0x7 => {
                // impl addi  op $r1 |-- value --| range(0--127)
                println!("addi");
                println!("reg before: {}", self.reg[b1 as usize]);
                let imm = ((b2 as u8) << 4) | b2 as u8;
                self.reg[b1 as usize] = imm;
                println!("reg after: {}", self.reg[b1 as usize]);
            },
            /*
              
            0x8 => println!("1"),
            0x9 => println!("2"),
            0xA => println!("1"),
            0xB => println!("2"), 
            */ 
            _ => println!("else"),
        }
    }

}

