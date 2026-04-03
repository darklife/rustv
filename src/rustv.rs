// RUST-V Simulator - April Fools 2026 Edition 🦀
// The first RISC-V "processor" fully designed in Rust (sort of... it's a 20-line toy)
// Thanks for Grok@xAI!

use std::process;

const MEMORY_SIZE: usize = 1024; // 1KB of glorious Rust-managed RAM

struct RustV {
    regs: [u32; 32], // x0..x31
    pc: u32,
    memory: [u8; MEMORY_SIZE],
}

impl RustV {
    fn new() -> Self {
        RustV {
            regs: [0; 32],
            pc: 0,
            memory: [0; MEMORY_SIZE],
        }
    }

    fn load_program(&mut self, program: &[u32]) {
        for (i, &instr) in program.iter().enumerate() {
            let addr = i * 4;
            if addr + 3 < MEMORY_SIZE {
                self.memory[addr]     = (instr & 0xFF) as u8;
                self.memory[addr + 1] = ((instr >> 8) & 0xFF) as u8;
                self.memory[addr + 2] = ((instr >> 16) & 0xFF) as u8;
                self.memory[addr + 3] = ((instr >> 24) & 0xFF) as u8;
            }
        }
    }

    fn fetch(&self) -> u32 {
        let addr = self.pc as usize;
        if addr + 3 >= MEMORY_SIZE {
            println!("Segmentation fault (core dumped)");
            process::exit(139);
        }
        let mut instr = 0u32;
        for i in 0..4 {
            instr |= (self.memory[addr + i] as u32) << (i * 8);
        }
        instr
    }

    fn execute(&mut self, instr: u32) -> bool {
        let opcode = instr & 0x7F;
        let rd = ((instr >> 7) & 0x1F) as usize;
        let rs1 = ((instr >> 15) & 0x1F) as usize;
        let rs2 = ((instr >> 20) & 0x1F) as usize;
        let funct3 = ((instr >> 12) & 0x7) as u32;
        let funct7 = (instr >> 25) & 0x7F;

        match opcode {
            0x13 => { // OP-IMM (ADDI etc.)
                if funct3 == 0 { // ADDI
                    let imm = ((instr >> 20) as i32) << 20 >> 20; // 12-bit sign extend
                    self.regs[rd] = self.regs[rs1].wrapping_add(imm as u32);
                    if rd == 0 { self.regs[0] = 0; } // x0 is always zero
                    true
                } else {
                    false
                }
            }
            0x33 => { // OP (ADD etc.)
                if funct3 == 0 && funct7 == 0 { // ADD
                    self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    if rd == 0 { self.regs[0] = 0; }
                    true
                } else {
                    false
                }
            }
            _ => false, // everything else = illegal
        }
    }

    fn step(&mut self) {
        let instr = self.fetch();
        if !self.execute(instr) {
            println!("Segmentation fault (core dumped) - unknown instruction 0x{:08x} at pc=0x{:08x}", instr, self.pc);
            process::exit(139);
        }
        self.pc += 4;
    }

    fn dump_regs(&self) {
        println!("\nRegister dump (after execution):");
        for i in 0..32 {
            if i % 8 == 0 && i != 0 { println!(); }
            print!("x{:02}={:08x} ", i, self.regs[i]);
        }
        println!();
    }
}

fn main() {
    println!("🦀 RUST-V Simulator v0.1 - the real April Fools RISC-V in Rust! 🦀");
    let mut cpu = RustV::new();

    // Tiny demo program (runs exactly 3 instructions, then we could segfault)
    let program: [u32; 3] = [
        0x00a00093, // ADDI x1, x0, 10
        0x01400113, // ADDI x2, x0, 20
        0x002081b3, // ADD  x3, x1, x2   → x3 = 30
    ];

    cpu.load_program(&program);

    println!("Running 3 valid instructions...");
    for _ in 0..3 {
        cpu.step();
    }

    cpu.dump_regs();
    println!("\n✅ Success! x3 should be 0000001e (30 in decimal)");

    // Try uncommenting the next line to see the "real" segfault in action:
    println!("\nTrying one more instruction (invalid opcode)...");
    cpu.step();  // ← this will print "Segmentation fault (core dumped)" and exit
}
