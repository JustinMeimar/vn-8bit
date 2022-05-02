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