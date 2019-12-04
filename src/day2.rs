#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> String {
    let mut memory = vec![Default::default(); input.len()];
    memory.copy_from_slice(input);

    // Before running the program, replace position 1 with the
    // value 12 and replace position 2 with the value 2.
    memory[1] = 12;
    memory[2] = 2;

    execute(&mut memory);
    println!("updated memory: {:?}", memory);

    memory[0].to_string()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u32]) -> String {
    let mut memory = vec![Default::default(); input.len()];

    let mut noun: u32 = 0;
    let mut verb: u32 = 0;

    // Use some brute force to solve. It's only 100*100 iterations
    // maximum, so will be quite fast in reality.
    'outer: for n in 0..=99 {
        for v in 0..=99 {
            memory.copy_from_slice(input);

            memory[1] = n;
            memory[2] = v;

            execute(&mut memory);

            if memory[0] == 19_690_720 {
                noun = n;
                verb = v;
                break 'outer;
            }
        }
    }

    println!("noun: {:?} verb: {:?}", noun, verb);
    println!("output: {:?}", memory[0].to_string());

    (100 * noun + verb).to_string()
}

fn execute(memory: &mut [u32]) -> String {
    let mut pointer: usize = 0;

    while memory[pointer] != 99 && pointer < memory.len() {
        pointer += do_instruction(&mut *memory, pointer);
    }

    memory
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn do_instruction(memory: &mut [u32], pointer: usize) -> usize {
    match memory[pointer] {
        1 => {
            memory[memory[pointer + 3] as usize] =
                memory[memory[pointer + 1] as usize] + memory[memory[pointer + 2] as usize];
            4
        }
        2 => {
            memory[memory[pointer + 3] as usize] =
                memory[memory[pointer + 1] as usize] * memory[memory[pointer + 2] as usize];
            4
        }
        99 => 1,
        _ => panic!(),
    }
}

#[cfg(test)]
mod test_part1 {
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
}
