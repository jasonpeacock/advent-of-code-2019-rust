use std::io;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i32]) -> String {
    let mut memory = vec![Default::default(); input.len()];
    memory.copy_from_slice(input);

    println!("Enter value:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => run(&mut memory, input.trim().parse().unwrap()),
        Err(error) => panic!("read input error: {:?}", error),
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i32]) -> String {
    let mut memory = vec![Default::default(); input.len()];
    memory.copy_from_slice(input);

    println!("Enter value:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => run(&mut memory, input.trim().parse().unwrap()),
        Err(error) => panic!("read input error: {:?}", error),
    }
}

fn run(memory: &mut [i32], input_value: i32) -> String {
    let mut pointer: usize = 0;

    let mut output = "no value".to_string();
    while memory[pointer] != 99 && pointer < memory.len() {
        let input = if pointer == 0 {
            println!("Using input value: {:?}", input_value);
            Some(input_value)
        } else {
            None
        };

        if let Some(value) = do_instruction(memory, &mut pointer, input) {
            output = value;
        }
    }

    if memory[pointer] == 99 {
        println!("Program halted");
    }

    output
}

fn do_instruction(memory: &mut [i32], pointer: &mut usize, input: Option<i32>) -> Option<String> {
    // The opcode is the 2 right-most digits.
    let address = *pointer;
    let instruction = memory[address];

    let mut output = None;

    let opcode = instruction % 100;
    match opcode {
        1 => opcode_add(memory, pointer),
        2 => opcode_multiply(memory, pointer),
        3 => opcode_input(memory, pointer, input),
        4 => output = Some(opcode_output(memory, pointer)),
        5 => opcode_jump_if_true(memory, pointer),
        6 => opcode_jump_if_false(memory, pointer),
        7 => opcode_less_than(memory, pointer),
        8 => opcode_equals(memory, pointer),
        99 => {
            *pointer += 1;
        }
        _ => panic!("Unknown opcode: {:?}", opcode),
    }

    output
}

// 1,a,b,c: Result of (a + b) is stored at index c.
// Return position of next address, which is +4 indexes.
fn opcode_add(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    println!(
        "@{:?} [{:?},{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2],
        memory[address + 3]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    let value_p3 = memory[address + 3];

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?}) => {:?} ({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3, memory[value_p3 as usize]
    );

    memory[value_p3 as usize] = value_p1 + value_p2;

    *pointer += 4;
}

#[cfg(test)]
mod opcode_add {
    use super::*;

    #[test]
    fn position_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("1,1,2,0,99");
        opcode_add(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "3,1,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("1101,2,3,0,99");
        opcode_add(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "5,2,3,0,99");
        assert_eq!(pointer, 4);
    }
}

fn opcode_multiply(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 2,a,b,c: Result of (a * b) is stored at index c.
    // Return position of next address, which is +4 indexes.
    println!(
        "@{:?} [{:?},{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2],
        memory[address + 3]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    let value_p3 = memory[address + 3];

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?}) => {:?} ({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3, memory[value_p3 as usize]
    );

    memory[value_p3 as usize] = value_p1 * value_p2;

    *pointer += 4;
}

#[cfg(test)]
mod opcode_multiply {
    use super::*;

    #[test]
    fn position_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("2,2,2,0,99");
        opcode_multiply(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "4,2,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("1102,2,3,0,99");
        opcode_multiply(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "6,2,3,0,99");
        assert_eq!(pointer, 4);
    }
}

fn opcode_input(memory: &mut [i32], pointer: &mut usize, input: Option<i32>) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 3,a: Read input and store at index a.
    println!(
        "@{:?} [{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1]
    );

    let value_p1 = memory[address + 1];

    match input {
        Some(value) => {
            println!(
                "\t{:?}: {:?} => {:?} ({:?})",
                opcode, value, value_p1, memory[value_p1 as usize]
            );

            memory[value_p1 as usize] = value;
        }
        None => panic!("Missing input value"),
    }

    *pointer += 2;
}

#[cfg(test)]
mod opcode_input {
    use super::*;

    #[test]
    fn position_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("3,2,0,99");
        opcode_input(&mut memory, &mut pointer, Some(1));
        assert_eq!(_memory_to_string(&memory), "3,2,1,99");
        assert_eq!(pointer, 2);
    }

    #[test]
    fn immediate_mode() {
        // Same behavior as position mode.
        let mut pointer = 0;
        let mut memory = input_generator("103,2,0,99");
        opcode_input(&mut memory, &mut pointer, Some(1));
        assert_eq!(_memory_to_string(&memory), "103,2,1,99");
        assert_eq!(pointer, 2);
    }
}

fn opcode_output(memory: &mut [i32], pointer: &mut usize) -> String {
    let address = *pointer;
    let instruction = memory[address];

    // 4,a: Echo the value at index a.
    println!(
        "@{:?} [{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    println!("\tTest result: {:?}", value_p1);

    *pointer += 2;

    value_p1.to_string()
}

#[cfg(test)]
mod opcode_output {
    use super::*;

    #[test]
    fn position_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("4,2,1,99");
        let output = opcode_output(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "4,2,1,99");
        assert_eq!(output, "1");
        assert_eq!(pointer, 2);
    }

    #[test]
    fn immediate_mode() {
        let mut pointer = 0;
        let mut memory = input_generator("104,2,1,99");
        let output = opcode_output(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "104,2,1,99");
        assert_eq!(output, "2");
        assert_eq!(pointer, 2);
    }
}

fn opcode_jump_if_true(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 5,a,b: If (a != 0), return b.
    println!(
        "@{:?} [{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2
    );

    if value_p1 != 0 {
        *pointer = value_p2 as usize;
    } else {
        *pointer += 3;
    }
}

#[cfg(test)]
mod opcode_jump_if_true {
    use super::*;

    #[test]
    fn position_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("5,2,1,99");
        opcode_jump_if_true(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "5,2,1,99");
        assert_eq!(pointer, 2);
    }

    #[test]
    fn position_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("5,2,0,99");
        opcode_jump_if_true(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "5,2,0,99");
        assert_eq!(pointer, 3);
    }

    #[test]
    fn immediate_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("1105,2,0,99");
        opcode_jump_if_true(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1105,2,0,99");
        assert_eq!(pointer, 0);
    }

    #[test]
    fn immediate_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("1105,0,0,99");
        opcode_jump_if_true(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1105,0,0,99");
        assert_eq!(pointer, 3);
    }
}

fn opcode_jump_if_false(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 6,a,b: If (a == 0), return b.
    println!(
        "@{:?} [{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2
    );

    if value_p1 == 0 {
        *pointer = value_p2 as usize;
    } else {
        *pointer += 3;
    }
}

#[cfg(test)]
mod opcode_jump_if_false {
    use super::*;

    #[test]
    fn position_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("6,2,1,99");
        opcode_jump_if_false(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "6,2,1,99");
        assert_eq!(pointer, 3);
    }

    #[test]
    fn position_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("6,2,0,99");
        opcode_jump_if_false(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "6,2,0,99");
        assert_eq!(pointer, 6);
    }

    #[test]
    fn immediate_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("1106,2,0,99");
        opcode_jump_if_false(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1106,2,0,99");
        assert_eq!(pointer, 3);
    }

    #[test]
    fn immediate_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("1106,0,0,99");
        opcode_jump_if_false(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1106,0,0,99");
        assert_eq!(pointer, 0);
    }
}

fn opcode_less_than(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 7,a,b,c: If (a < b), store 1 at index c, else store 0 at index c.
    println!(
        "@{:?} [{:?},{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2],
        memory[address + 3]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    let value_p3 = memory[address + 3];

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?}) => {:?} ({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3, memory[value_p3 as usize]
    );

    memory[value_p3 as usize] = if value_p1 < value_p2 { 1 } else { 0 };

    *pointer += 4;
}

#[cfg(test)]
mod opcode_less_than {
    use super::*;

    #[test]
    fn position_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("7,0,4,0,99");
        opcode_less_than(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1,0,4,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn position_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("7,0,2,0,99");
        opcode_less_than(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "0,0,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("1107,0,2,0,99");
        opcode_less_than(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1,0,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("1107,2,0,0,99");
        opcode_less_than(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "0,2,0,0,99");
        assert_eq!(pointer, 4);
    }
}

fn opcode_equals(memory: &mut [i32], pointer: &mut usize) {
    let address = *pointer;
    let instruction = memory[address];

    let opcode = instruction % 100;

    // 8,a,b,c: If (a == b), store 1 at index c, else store 0 at index c.
    println!(
        "@{:?} [{:?},{:?},{:?},{:?}]",
        address,
        memory[address],
        memory[address + 1],
        memory[address + 2],
        memory[address + 3]
    );

    let mode_p1 = get_parameter_mode(instruction, 1);
    let value_p1 = get_parameter_value(mode_p1, address + 1, memory);

    let mode_p2 = get_parameter_mode(instruction, 2);
    let value_p2 = get_parameter_value(mode_p2, address + 2, memory);

    let value_p3 = memory[address + 3];

    println!(
        "\t{:?}: {:?}({:?}) {:?}({:?}) => {:?} ({:?})",
        opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3, memory[value_p3 as usize]
    );

    memory[value_p3 as usize] = if value_p1 == value_p2 { 1 } else { 0 };

    *pointer += 4;
}

#[cfg(test)]
mod opcode_equals {
    use super::*;

    #[test]
    fn position_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("8,0,0,0,99");
        opcode_equals(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1,0,0,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn position_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("8,0,2,0,99");
        opcode_equals(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "0,0,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode_true() {
        let mut pointer = 0;
        let mut memory = input_generator("1108,2,2,0,99");
        opcode_equals(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "1,2,2,0,99");
        assert_eq!(pointer, 4);
    }

    #[test]
    fn immediate_mode_false() {
        let mut pointer = 0;
        let mut memory = input_generator("1108,2,3,0,99");
        opcode_equals(&mut memory, &mut pointer);
        assert_eq!(_memory_to_string(&memory), "0,2,3,0,99");
        assert_eq!(pointer, 4);
    }
}

fn get_parameter_value(mode: i32, address: usize, memory: &[i32]) -> i32 {
    if mode == 0 {
        // Need to dereference the value at the address to find the actual value.
        memory[memory[address] as usize]
    } else if mode == 1 {
        // The value at the address is the value.
        memory[address]
    } else {
        panic!("unknown mode: {:?} at address: {:?}", mode, address)
    }
}

// 1-based positioning.
fn get_parameter_mode(opcode: i32, position: i32) -> i32 {
    let small_offset = 10i32.saturating_pow((position + 1i32) as u32);
    let large_offset = 10i32.saturating_pow((position + 2i32) as u32);

    (opcode % large_offset - opcode % small_offset) / small_offset
}

fn _memory_to_string(memory: &[i32]) -> String {
    memory
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod part2 {
    use super::*;

    #[test]
    fn example_1() {
        // 1 if equal to 8, otherwise 0.
        assert_eq!(
            run(&mut input_generator("3,9,8,9,10,9,4,9,99,-1,8"), 1),
            "0"
        );
        assert_eq!(
            run(&mut input_generator("3,9,8,9,10,9,4,9,99,-1,8"), 9),
            "0"
        );
        assert_eq!(
            run(&mut input_generator("3,9,8,9,10,9,4,9,99,-1,8"), 8),
            "1"
        );
    }

    #[test]
    fn example_2() {
        // 1 if less than 8, otherwise 0.
        assert_eq!(
            run(&mut input_generator("3,9,7,9,10,9,4,9,99,-1,8"), 1),
            "1"
        );
        assert_eq!(
            run(&mut input_generator("3,9,7,9,10,9,4,9,99,-1,8"), 9),
            "0"
        );
        assert_eq!(
            run(&mut input_generator("3,9,7,9,10,9,4,9,99,-1,8"), 8),
            "0"
        );
    }

    #[test]
    fn example_3() {
        // 1 if equal to 8, otherwise 0.
        assert_eq!(run(&mut input_generator("3,3,1108,-1,8,3,4,3,99"), 1), "0");
        assert_eq!(run(&mut input_generator("3,3,1108,-1,8,3,4,3,99"), 9), "0");
        assert_eq!(run(&mut input_generator("3,3,1108,-1,8,3,4,3,99"), 8), "1");
    }

    #[test]
    fn example_4() {
        // 1 if less than 8, otherwise 0.
        assert_eq!(run(&mut input_generator("3,3,1107,-1,8,3,4,3,99"), 1), "1");
        assert_eq!(run(&mut input_generator("3,3,1107,-1,8,3,4,3,99"), 9), "0");
        assert_eq!(run(&mut input_generator("3,3,1107,-1,8,3,4,3,99"), 8), "0");
    }

    #[test]
    fn example_5() {
        // 0 if 0, otherwise 1.
        assert_eq!(
            run(
                &mut input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
                1
            ),
            "1"
        );
        assert_eq!(
            run(
                &mut input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
                -1
            ),
            "1"
        );
        assert_eq!(
            run(
                &mut input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
                0
            ),
            "0"
        );
    }

    #[test]
    fn example_6() {
        // 0 if 0, otherwise 1.
        assert_eq!(
            run(
                &mut input_generator("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"),
                1
            ),
            "1"
        );
        assert_eq!(
            run(
                &mut input_generator("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"),
                0
            ),
            "0"
        );
    }

    #[test]
    fn example_7() {
        // 0 if 0, otherwise 1.
        assert_eq!(
            run(
                &mut input_generator(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                1
            ),
            "999"
        );
        assert_eq!(
            run(
                &mut input_generator(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                9
            ),
            "1001"
        );
        assert_eq!(
            run(
                &mut input_generator(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                8
            ),
            "1000"
        );
    }
}

#[cfg(test)]
mod get_parameter_mode {
    use super::*;

    #[test]
    fn parameter_1() {
        assert_eq!(get_parameter_mode(1234, 1), 2);
        assert_eq!(get_parameter_mode(1002, 1), 0);
    }

    #[test]
    fn parameter_2() {
        assert_eq!(get_parameter_mode(1234, 2), 1);
        assert_eq!(get_parameter_mode(1002, 2), 1);
    }

    #[test]
    fn parameter_too_large() {
        assert_eq!(get_parameter_mode(234, 2), 0);
        assert_eq!(get_parameter_mode(1234, 3), 0);
        assert_eq!(get_parameter_mode(1234, 10), 0);
    }
}

#[cfg(test)]
mod part1 {
    use super::*;

    #[test]
    fn example_1() {
        let mut memory = input_generator("1,0,0,0,99");
        assert_eq!(run(&mut memory, 1), "");
        assert_eq!(_memory_to_string(&memory), "2,0,0,0,99");
    }

    #[test]
    fn example_2() {
        let mut memory = input_generator("2,3,0,3,99");
        assert_eq!(run(&mut memory, 1), "");
        assert_eq!(_memory_to_string(&memory), "2,3,0,6,99");
    }

    #[test]
    fn example_3() {
        let mut memory = input_generator("2,4,4,5,99,0");
        assert_eq!(run(&mut memory, 1), "");
        assert_eq!(_memory_to_string(&memory), "2,4,4,5,99,9801");
    }

    #[test]
    fn example_4() {
        let mut memory = input_generator("1,1,1,4,99,5,6,0,99");
        assert_eq!(run(&mut memory, 1), "");
        assert_eq!(_memory_to_string(&memory), "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn example_5() {
        let mut memory = input_generator("1002,4,3,4,33");
        assert_eq!(run(&mut memory, 1), "");
        assert_eq!(_memory_to_string(&memory), "1002,4,3,4,99");
    }

    #[test]
    fn example_input_output() {
        let mut memory = input_generator("3,0,4,0,99");
        assert_eq!(run(&mut memory, 1), "1");
        assert_eq!(_memory_to_string(&memory), "1,0,4,0,99");
    }
}
