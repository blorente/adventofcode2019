// # Notes
// - Since the program can self-modify, we have to interpret opcodes whenever we fetch 
// . (can't parse the program).

// # TODO
// - Depending on future enhancements, the logic to indirectly fetch an operand can be abstracted

mod instructions;
use instructions::{InstructionSet, Memory, Add, Mul, Terminate};

fn load_memory(input_file: &str) -> Result<Memory, String> {
    let input_string = std::fs::read_to_string(input_file).map_err(|e| format!("Error while reading the input file: {}", e))?;
    let elements = input_string.trim().split(",");
    let data: Result<Vec<usize>, String> = elements
                .map(|s| s.parse::<usize>().map_err(|e| format!("Error when parsing input: {}", e)))
                .collect();
    data.map(|contents: Vec<usize>| Box::new(contents))
}

fn run(instructions: &InstructionSet, memory: &mut Memory) {
    let mut pc = 0;
    while pc != std::usize::MAX {
        let opcode = memory[pc];
        pc += 1;
        pc = instructions.instructions
                .get(&opcode)
                .map(|inst| inst.execute(memory, pc))
                .expect(&format!("Unrecognized opcode {}", opcode))
    }
}

fn advent_01(instructions: InstructionSet) -> Result<(usize, usize), String> {
    let original_memory: Memory = load_memory("input01.txt")?;
    for in1 in 0..99 {
        for in2 in 0..99 {
            let mut memory = original_memory.clone();
            memory[1] = in1;
            memory[2] = in2;
            run(&instructions, &mut memory);
            if memory[0] == 19690720 {
                return Ok((in1, in2));
            }
        }
    }
    Err("Couldn't find the right operands!".to_string())
}

fn main() -> Result<(), String> {
    let instructions = InstructionSet::new(vec![
        (1, Box::new(Add {})),
        (2, Box::new(Mul {})),
        (99, Box::new(Terminate {})),
    ]);

    let res = advent_01(instructions);
    println!("Result is {:?}", res);
    res.map(|_| ())
}
