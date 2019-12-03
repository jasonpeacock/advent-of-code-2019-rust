type Mass = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Mass> {
    input
        .lines()
        .map(|l| {
            let mass = l.trim().parse().unwrap();
            (mass)
        })
        .collect()
}

pub fn calc_fuel(mass: Mass) -> i32 {
    ((mass as f32 / 3.).trunc() as i32) - 2
}

pub fn calc_fuel_recurse(mass: Mass) -> i32 {
    let fuel = calc_fuel(mass);

    if fuel > 0 {
        fuel + calc_fuel_recurse(fuel)
    } else {
        0
    }
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Mass]) -> i32 {
    input.iter().map(|&mass| calc_fuel(mass)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Mass]) -> i32 {
    input.iter().map(|&mass| calc_fuel_recurse(mass)).sum()
}

#[cfg(test)]
mod test_part1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(&input_generator("12")), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part1(&input_generator("14")), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_part1(&input_generator("1969")), 654);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_part1(&input_generator("100756")), 33583);
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part2(&input_generator("14")), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(&input_generator("1969")), 966);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_part2(&input_generator("100756")), 50346);
    }
}
