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
    let mut dest = vec![Default::default(); input.len()];
    dest.copy_from_slice(input);

    // XXX
    dest[1] = 12;
    dest[2] = 2;

    let mut index: usize = 0;
    while dest[index] != 99 && index < dest.len() {
        dest = execute(&mut dest, index).to_vec();
        index += 4;
    }

    slice_to_string(&dest)
}

fn execute(input: &mut [u32], index: usize) -> &[u32] {
    if input[index] == 1 {
        input[input[index + 3] as usize] =
            input[input[index + 1] as usize] + input[input[index + 2] as usize];
        input
    } else if input[index] == 2 {
        input[input[index + 3] as usize] =
            input[input[index + 1] as usize] * input[input[index + 2] as usize];
        input
    } else {
        panic!()
    }
}

fn slice_to_string(input: &[u32]) -> String {
    input
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod test_part1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(&input_generator("1,0,0,0,99")), "2,0,0,0,99");
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part1(&input_generator("2,3,0,3,99")), "2,3,0,6,99");
    }

    #[test]
    fn example_3() {
        assert_eq!(
            solve_part1(&input_generator("2,4,4,5,99,0")),
            "2,4,4,5,99,9801"
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            solve_part1(&input_generator("1,1,1,4,99,5,6,0,99")),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
