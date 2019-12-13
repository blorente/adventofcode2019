use std::collections::HashMap;

pub type PC = usize;
pub type Memcell = usize;
pub type Memory = Box<Vec<Memcell>>;

pub trait Instruction {
    fn execute(&self, memory: &mut Memory, current_pc: PC) -> PC;
    fn opcode_size(&self) -> Memcell {4} // 4 is the default
    fn consume_one(&self, memory: &mut Memory, current_pc: PC) -> (Memcell, PC) { 
        let val = memory[current_pc];
        (val, current_pc + 1)
    }
}

pub struct Add {}
impl Instruction for Add {
    fn execute(&self, memory: &mut Memory, current_pc: PC) -> PC {
        let pc = current_pc;
        let (left_operand_addr, pc) = self.consume_one(memory, pc);
        let (right_operand_addr, pc) = self.consume_one(memory, pc);
        let (ret_addr, pc) = self.consume_one(memory, pc);
        let res = memory[left_operand_addr] + memory[right_operand_addr];
        memory[ret_addr] = res;
        pc
    }
}

pub struct Mul {}
impl Instruction for Mul {
    fn execute(&self, memory: &mut Memory, current_pc: PC) -> PC {
        let pc = current_pc;
        let (left_operand_addr, pc) = self.consume_one(memory, pc);
        let (right_operand_addr, pc) = self.consume_one(memory, pc);
        let (ret_addr, pc) = self.consume_one(memory, pc);
        let res = memory[left_operand_addr] * memory[right_operand_addr];
        memory[ret_addr] = res;
        pc
    }
}

pub struct Terminate {}
impl Instruction for Terminate {
    fn execute(&self, _memory: &mut Memory, _current_pc: PC) -> PC {
        // LEAKING ABSTRACTION!
        std::usize::MAX
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