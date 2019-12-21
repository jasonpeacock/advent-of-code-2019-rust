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

    execute(&mut memory);
    println!("updated memory: {:?}", memory);

    memory[0].to_string()
}

fn execute(memory: &mut [i32]) -> String {
    let mut pointer: usize = 0;

    while memory[pointer] != 99 && pointer < memory.len() {
        pointer += do_instruction(memory, pointer);
    }

    if memory[pointer] == 99 {
        println!("Program halted");
    }

    memory
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn do_instruction(memory: &mut [i32], pointer: usize) -> usize {
    // The opcode is the 2 right-most digits.
    let instruction = memory[pointer];

    let opcode = instruction % 100;
    match opcode {
        1 => {
            // 1,a,b,c: Result of (a + b) is stored at index c.
            // Return position of next pointer, which is +4 indexes.
            println!(
                "[{:?},{:?},{:?},{:?}]",
                memory[pointer],
                memory[pointer + 1],
                memory[pointer + 2],
                memory[pointer + 3]
            );

            let mode_p1 = get_parameter_mode(instruction, 1);
            let value_p1 = get_parameter_value(mode_p1, pointer + 1, memory);

            let mode_p2 = get_parameter_mode(instruction, 2);
            let value_p2 = get_parameter_value(mode_p2, pointer + 2, memory);

            let value_p3 = memory[pointer + 3];

            println!(
                "{:?}: {:?}({:?}) {:?}({:?}) => {:?}",
                opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3
            );

            memory[value_p3 as usize] = value_p1 + value_p2;

            4
        }
        2 => {
            // 2,a,b,c: Result of (a * b) is stored at index c.
            // Return position of next pointer, which is +4 indexes.
            println!(
                "[{:?},{:?},{:?},{:?}]",
                memory[pointer],
                memory[pointer + 1],
                memory[pointer + 2],
                memory[pointer + 3]
            );

            let mode_p1 = get_parameter_mode(instruction, 1);
            let value_p1 = get_parameter_value(mode_p1, pointer + 1, memory);

            let mode_p2 = get_parameter_mode(instruction, 2);
            let value_p2 = get_parameter_value(mode_p2, pointer + 2, memory);

            let value_p3 = memory[pointer + 3];

            println!(
                "{:?}: {:?}({:?}) {:?}({:?}) => {:?}",
                opcode, value_p1, mode_p1, value_p2, mode_p2, value_p3
            );

            memory[value_p3 as usize] = value_p1 * value_p2;

            4
        }
        3 => {
            // 3,a: Read input and store at index a.
            println!("[{:?},{:?}]", memory[pointer], memory[pointer + 1]);

            let value_p1 = memory[pointer + 1];

            println!("Enter value:");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    memory[value_p1 as usize] = input.trim().parse().unwrap();
                }
                Err(error) => panic!("read input error: {:?}", error),
            }

            2
        }
        4 => {
            // 4,a: Echo the value at index a.
            println!("[{:?},{:?}]", memory[pointer], memory[pointer + 1]);

            let mode_p1 = get_parameter_mode(instruction, 1);
            let value_p1 = get_parameter_value(mode_p1, pointer + 1, memory);

            println!("Test result: {:?}", value_p1);

            2
        }
        99 => 1,
        _ => panic!("Unknown opcode: {:?}", opcode),
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
        assert_eq!(execute(&mut input_generator("1,0,0,0,99")), "2,0,0,0,99");
    }

    #[test]
    fn example_2() {
        assert_eq!(execute(&mut input_generator("2,3,0,3,99")), "2,3,0,6,99");
    }

    #[test]
    fn example_3() {
        assert_eq!(
            execute(&mut input_generator("2,4,4,5,99,0")),
            "2,4,4,5,99,9801"
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            execute(&mut input_generator("1,1,1,4,99,5,6,0,99")),
            "30,1,1,4,2,5,6,0,99"
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            execute(&mut input_generator("1002,4,3,4,33")),
            "1002,4,3,4,99"
        );
    }

    #[test]
    fn example_input_output() {
        assert_eq!(execute(&mut input_generator("3,0,4,0,99")), "1,0,4,0,99");
    }
}
