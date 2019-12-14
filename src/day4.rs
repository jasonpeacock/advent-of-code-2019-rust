#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (u32, u32) {
    let range_list: Vec<u32> = input.split('-').map(|d| d.parse().unwrap()).collect();
    (range_list[0], range_list[1])
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(u32, u32)) -> u32 {
    let mut count: u32 = 0;
    for number in input.0..input.1 {
        if validate(number, false) {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(u32, u32)) -> u32 {
    let mut count: u32 = 0;
    for number in input.0..input.1 {
        if validate(number, true) {
            count += 1;
        }
    }

    count
}

fn validate(number: u32, part2: bool) -> bool {
    let mut prev_digit: u32 = 0;
    let mut has_double = false;
    let mut valid_double = false;
    let mut is_decreasing = false;
    let mut duplicate_count = 1;
    number.to_string().chars().for_each(|char| {
        let digit = char.to_digit(10).unwrap();

        if digit < prev_digit {
            is_decreasing = true;
        }

        if digit == prev_digit {
            duplicate_count += 1;

            if part2 {
                // Part 2 has stricter rules for duplicate
                // counting.
                if duplicate_count == 2 {
                    has_double = true;
                }

                if duplicate_count > 2 {
                    has_double = false;
                }
            } else if duplicate_count >= 2 {
                has_double = true;
            }
        } else {
            // Switching to a new digit.
            duplicate_count = 1;

            // Remember if there was *any* valid doubles,
            // as we may encounter an invalid double next.
            if has_double {
                valid_double = true;
            }
        }

        prev_digit = digit;
    });

    // Handle last-digit case where the valid double is at
    // the end of the string.
    if has_double {
        valid_double = true;
    }

    valid_double && !is_decreasing
}

#[cfg(test)]
mod test_part1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(validate(111111, false), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(validate(223450, false), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(validate(123789, false), false);
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(validate(112233, true), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(validate(123444, true), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(validate(111122, true), true);
    }

    #[test]
    fn example_4() {
        assert_eq!(validate(111223, true), true);
    }
}
