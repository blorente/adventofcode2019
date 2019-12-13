use std::collections::HashMap;
use std::io::{Read, BufReader, BufRead};

pub type PC = usize;
pub type Memcell = usize;
pub type Memory = Box<Vec<Memcell>>;

pub trait Instruction {
    fn execute(&mut self, memory: &mut Memory, current_pc: PC) -> Result<PC, String>;
    fn opcode_size(&self) -> Memcell {4} // 4 is the default
    fn consume_one(&self, memory: &mut Memory, current_pc: PC) -> (Memcell, PC) { 
        let val = memory[current_pc];
        (val, current_pc + 1)
    }
}

pub struct Add {}
impl Instruction for Add {
    fn execute(&mut self, memory: &mut Memory, current_pc: PC) -> Result<PC, String> {
        let pc = current_pc;
        let (left_operand_addr, pc) = self.consume_one(memory, pc);
        let (right_operand_addr, pc) = self.consume_one(memory, pc);
        let (ret_addr, pc) = self.consume_one(memory, pc);
        let res = memory[left_operand_addr] + memory[right_operand_addr];
        memory[ret_addr] = res;
        Ok(pc)
    }
}

pub struct Mul {}
impl Instruction for Mul {
    fn execute(&mut self, memory: &mut Memory, current_pc: PC) -> Result<PC, String> {
        let pc = current_pc;
        let (left_operand_addr, pc) = self.consume_one(memory, pc);
        let (right_operand_addr, pc) = self.consume_one(memory, pc);
        let (ret_addr, pc) = self.consume_one(memory, pc);
        let res = memory[left_operand_addr] * memory[right_operand_addr];
        memory[ret_addr] = res;
        Ok(pc)
    }
}

pub struct Terminate {}
impl Instruction for Terminate {
    fn execute(&mut self, _memory: &mut Memory, _current_pc: PC) -> Result<PC, String> {
        // LEAKING ABSTRACTION!
        Ok(std::usize::MAX)
    }
}

pub struct Input {
    pub stream: Box<dyn Iterator<Item = Memcell>>,
}
impl Instruction for Input {
    fn execute(&mut self, memory: &mut Memory, current_pc: PC) -> Result<PC, String> {
        let pc = current_pc;
        let (ret_addr, pc) = self.consume_one(memory, pc);
        let val: Memcell = self.stream.next().ok_or(format!("Error fetching the next element in stream!"))?;
        memory[ret_addr] = val;
        Ok(pc)
    }
}

pub struct Output {}
impl Instruction for Output {
    fn execute(&mut self, memory: &mut Memory, current_pc: PC) -> Result<PC, String> {
        let pc = current_pc;
        let (addr, pc) = self.consume_one(memory, pc);
        let val = memory[addr];
        println!("{}", val);
        Ok(pc)
    }
}

pub struct InstructionSet {
    pub instructions: HashMap<usize, Box<dyn Instruction>>
}

impl InstructionSet {
  pub fn new(instructions: Vec<(usize, Box<dyn Instruction>)>) -> Self {
      let mut instruction_set = InstructionSet {
          instructions: HashMap::new(),
      };
      for (opcode, inst) in instructions {
          instruction_set.instructions.insert(opcode, inst);
      }
      instruction_set
  }
}