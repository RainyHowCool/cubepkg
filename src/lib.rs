use std::fs;

#[derive(Debug)]
pub struct Operation {
    opcode: u8,
    opt1: i32,
    opt2: i32,
}

#[derive(Debug, Clone)]
#[allow(non_snake_case, unused)] // I allowed lmao
pub struct Register {
    R0: usize,
    R1: usize,
    R2: usize,
    R3: usize,
    R4: usize,
    R5: usize,
    R6: usize,
    R7: usize,
    R8: usize,
    R9: usize,
    R10: usize,
    R11: usize,
    R12: usize,
    R13: usize,
    R14: usize,
    R15: usize,
    IP: usize,
    SEG: usize,
}

impl Register {
    pub fn new() -> Self {
        Self {
            R0: 0,
            R1: 0,
            R2: 0,
            R3: 0,
            R4: 0,
            R5: 0,
            R6: 0,
            R7: 0,
            R8: 0,
            R9: 0,
            R10: 0,
            R11: 0,
            R12: 0,
            R13: 0,
            R14: 0,
            R15: 0,
            IP: 0,
            SEG: 0,
        }
    }
}

impl Operation {
    pub fn new(opcode: u8, opt1: i32, opt2: i32) -> Self {
        Self {
            opcode,
            opt1,
            opt2
        }
    }

    pub fn generate(&self) -> Vec<u8> {
        if self.opcode <= 5 {
            // TODO: 加紧修理这坨💩
            // NOTE: 给人类9178年都写不出这种代码
            return vec![0x32, self.opcode << 4 | self.opt1 as u8, 
                    (self.opt2 << 24 >> 24) as u8,
                    (self.opt2 >> 8 << 24 >> 24) as u8,
                    (self.opt2 >> 16 << 24 >> 24) as u8,
                    (self.opt2 >> 24) as u8];
        } else if self.opcode <= 0xA {
            return vec![self.opcode << 4 | self.opt1 as u8,
                self.opt2 as u8]
        } else if self.opcode >= 0xF0 {
            return vec![self.opcode];
        }
        vec![]
    }
}

pub struct VM {
    memory: i32,
    code_seg: Vec<u8>,
    data_seg: Vec<u8>,
    debug: bool,
}

impl VM {
    pub fn from_file(memory: i32, debug: bool, name: String) -> Result<Self, u8> {
        let mut code_seg: Vec<u8> = vec![];
        let mut data_seg: Vec<u8> = vec![];
        let raw: Vec<u8> = fs::read(name).unwrap();
        // 1. Check magic number
        if raw.len() < 32 {
            eprintln!("ERR: Invaild size {}", raw.len());
            return Err(1);
        }
        let magic_number: &[u8] = &raw[0..4];
        let true_magic_number: &[u8] = &[0x1e, 0x55, 0xc6, 0xb3];
        if magic_number != true_magic_number {
            eprintln!("{:?} != {:?}!", magic_number, true_magic_number);
            return Err(2);
        }
        // 2. Check Version
        let version_number: &[u8] = &raw[4..6];
        let true_version_number: &[u8] = &[0x00, 0x00];
        if version_number != true_version_number {
            return Err(3);
        }
        // 3. Uncompress
        // TODO: Not implemented yet
        // 4. Parse Segment Table
        let segment_count: u8 = raw[8];
        let mut prev_segtable_len: usize = 0;
        for _i in 0..segment_count {
            let execuable: u8 = raw[{ prev_segtable_len += 1; (8 + prev_segtable_len) as usize}]; // wtf
            // Jump name
            prev_segtable_len += 1 + raw[(9 + prev_segtable_len) as usize] as usize;
            println!("{}", prev_segtable_len);
            // Offest
            let mut seg_offest = prev_segtable_len + 9;
            let offest = raw[seg_offest] as u32 |
                        (raw[(seg_offest + 1) as usize] as u32) << 8 |
                        (raw[(seg_offest + 2) as usize] as u32) << 16 |
                        (raw[(seg_offest + 3) as usize] as u32) << 24;
            println!("off:{}", offest);
            prev_segtable_len += 4;
            seg_offest += 4;
            // Length
            let len = raw[seg_offest] as u32 |
                        (raw[(seg_offest + 1) as usize] as u32) << 8 |
                        (raw[(seg_offest + 2) as usize] as u32) << 16 |
                        (raw[(seg_offest + 3) as usize] as u32) << 24;
            prev_segtable_len += 4;
            println!("len:{}", len);
            // Execute
            if execuable == 1 {
                code_seg = raw[offest as usize..(offest + len) as usize].to_vec();
            } else {
                data_seg.extend(&raw[offest as usize..(offest + len) as usize]);
            }
        }
        Ok(Self {
            memory,
            code_seg,
            data_seg,
            debug,
        })
    }

    pub fn change_register_by_id(register: &mut Register, id: u8, value: usize) {
        match id {
            0 => register.R0 = value,
            1 => register.R1 = value,
            2 => register.R2 = value,
            3 => register.R3 = value,
            4 => register.R4 = value,
            5 => register.R5 = value,
            6 => register.R6 = value,
            7 => register.R7 = value,
            8 => register.R8 = value,
            9 => register.R9 = value,
            10 => register.R10 = value,
            11 => register.R11 = value,
            12 => register.R12 = value,
            13 => register.R13 = value,
            14 => register.R14 = value,
            15 => register.R15 = value,
            _ => todo!(),
        }
    }

    pub fn get_register_by_id(register: Register, id: u8) -> usize {
        match id {
            0 => register.R0,
            1 => register.R1,
            2 => register.R2,
            3 => register.R3,
            4 => register.R4,
            5 => register.R5,
            6 => register.R6,
            7 => register.R7,
            8 => register.R8,
            9 => register.R9,
            10 => register.R10,
            11 => register.R11,
            12 => register.R12,
            13 => register.R13,
            14 => register.R14,
            15 => register.R15,
            _ => { println!("{}", id); todo!(); },
        }
    }

    fn debugger(&self, register: &mut Register) {
        println!("{:?}", register);
    }

    pub fn get_u32(mem: &[u8], offest: usize) -> u32 {
        (mem[offest]) as u32 | 
            ((mem[offest + 1]) as u32) << 8 | 
            ((mem[offest + 2]) as u32) << 16 |
            ((mem[offest + 3]) as u32) << 24
    }

    pub fn run(&self) {
        let mut ram: Vec<u8> = Vec::with_capacity(self.memory as usize);
        // Copy data to RAM
        ram.extend(&self.code_seg);
        ram.extend(&self.data_seg);
        // Power on
        //let ram: &[u8] = &ram_unlimited[0..self.memory as usize];
        let mut register = Register::new();
        while register.IP <= self.memory as usize {
            if ram[register.IP] == 0x32 { register.IP += 1; continue; }
            let register_ref = register.clone();
            match ram[register.IP] >> 4 {
                0x01 => { 
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_u32(&ram, (register_ref.IP + 1) as usize) as usize);
                    register.IP += 4;
                },
                0x02 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) + 
                            Self::get_u32(&ram, (register_ref.IP + 1) as usize) as usize);
                    register.IP += 4;
                },
                0x03 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) - 
                            Self::get_u32(&ram, (register_ref.IP + 1) as usize) as usize);
                    register.IP += 4;
                },
                0x04 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) *
                            Self::get_u32(&ram, (register_ref.IP + 1) as usize) as usize);
                    register.IP += 4;
                },
                0x05 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) /
                            Self::get_u32(&ram, (register_ref.IP + 1) as usize) as usize);
                    register.IP += 4;  
                },
                0x06 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) +
                           Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP + 1]));
                    register.IP += 1;
                },                
                0x07 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) -
                           Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP + 1]));
                    register.IP += 1;
                },                
                0x08 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) *
                           Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP + 1]));
                    register.IP += 1;
                },
                0x09 => {
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP] << 4 >> 4) /
                           Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP + 1]));
                    register.IP += 1;
                },
                0x0A => { 
                    Self::change_register_by_id(&mut register, ram[register_ref.IP] << 4 >> 4, 
                        Self::get_register_by_id(register_ref.clone(), ram[register_ref.IP + 1]));
                    register.IP += 1;
                },
                0x0F => {
                    match ram[register.IP] << 4 >> 4 {
                        0 => return,
                        _ => eprintln!("Unknown opcode"),
                    }
                },
                _ => eprintln!("Unknown opcode"),
            }
            if self.debug { self.debugger(&mut register); }
            register.IP += 1;
        }
    }
}