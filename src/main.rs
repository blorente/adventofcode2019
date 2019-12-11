use std::collections::HashMap;
// # Notes
// - Since the program can self-modify, we have to interpret opcodes whenever we fetch 
// . (can't parse the program).

// # TODO
// - Depending on future enhancements, the logic to indirectly fetch an operand can be abstracted

type PC = usize;
type Memcell = usize;
type Memory = Box<Vec<Memcell>>;

trait Instruction {
    fn execute(&self, memory: &mut Memory, current_pc: PC) -> PC;
    fn opcode_size(&self) -> Memcell {4} // 4 is the default
    fn consume_one(&self, memory: &mut Memory, current_pc: PC) -> (Memcell, PC) { 
        let val = memory[current_pc];
        (val, current_pc + 1)
    }
}

struct Add {}
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

struct Mul {}
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

struct Terminate {}
impl Instruction for Terminate {
    fn execute(&self, _memory: &mut Memory, _current_pc: PC) -> PC {
        // LEAKING ABSTRACTION!
        std::usize::MAX
    }
}

struct InstructionSet {
    instructions: HashMap<usize, Box<dyn Instruction>>
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

fn load_memory(input_file: &str) -> Result<Memory, String> {
    let input_string = std::fs::read_to_string(input_file).map_err(|e| format!("Error while reading the input file: {}", e))?;
    let elements = input_string.trim().split(",");
    let data: Result<Vec<usize>, String> = elements
                .map(|s| s.parse::<usize>().map_err(|e| format!("Error when parsing input: {}", e)))
                .collect();
    data.map(|contents: Vec<usize>| Box::new(contents))
}

fn preprocess_memory(memory: &mut Memory) {
    memory[1] = 12;
    memory[2] = 2
}

fn main() -> Result<(), String> {
    //let mut memory: Memory = Box::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
    let mut memory: Memory = load_memory("input01.txt")?;

    preprocess_memory(&mut memory);

    let instructions = InstructionSet::new(vec![
        (1, Box::new(Add {})),
        (2, Box::new(Mul {})),
        (99, Box::new(Terminate {})),
    ]);

    let mut pc = 0;
    while pc != std::usize::MAX {
        let opcode = memory[pc];
        pc += 1;
        pc = instructions.instructions
                .get(&opcode)
                .map(|inst| inst.execute(&mut memory, pc))
                .expect(&format!("Unrecognized opcode {}", opcode))
    }
    
    println!("Resulting memory:\n{:#?}", memory);
    Ok(())
}
