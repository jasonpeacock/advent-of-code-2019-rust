use std::collections::HashMap;
use std::convert::TryInto;

type Directions = Vec<String>;
type Coordinate = (i32, i32);
type Coordinates = Vec<Coordinate>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Directions> {
    input
        .lines()
        .map(|l| l.trim().split(',').map(|d| d.parse().unwrap()).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Directions]) -> u32 {
    // println!("parsed input:\n\t{:?}", input);
    let paths = generate_paths(input);
    let intersections = find_intersections(&paths);

    // For each intersection of all wires, calculate the Manhattan Distance.
    let mut distances = intersections
        .iter()
        .filter_map(|(&k, &v)| if v == 1 { Some(k) } else { None })
        .map(manhattan_distance)
        .collect::<Vec<u32>>();

    distances.sort();

    // println!("distances: {:?}", distances);

    // Take the closest intersection.
    distances[0]
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Directions]) -> u32 {
    // println!("parsed input:\n\t{:?}", input);
    let paths = generate_paths(input);
    let intersections = find_intersections(&paths);

    let mut shortest_distance = std::u32::MAX;
    intersections
        .iter()
        .filter_map(|(&k, &v)| if v == 1 { Some(k) } else { None })
        .for_each(|intersection| {
            let mut total_distance = 0;
            paths.iter().for_each(|path| {
                total_distance += path
                    .iter()
                    .position(|&coordinate| coordinate == intersection)
                    .unwrap() as u32;
            });

            if total_distance < shortest_distance {
                shortest_distance = total_distance
            };
        });

    shortest_distance
}

fn generate_paths(directions: &[Directions]) -> Vec<Coordinates> {
    // Convert each direction into a path of coordinates that it has traversed.
    directions
        .iter()
        .map(|i| directions_into_coordinates(&i))
        .collect()
}

fn find_intersections(paths: &[Coordinates]) -> HashMap<Coordinate, u32> {
    /*
    for path in &paths {
        println!("path:\n\t{:?}", path);
    }
    */

    // Load each wire's path into the Map and mark when intersections are found.
    let mut intersections: HashMap<Coordinate, u32> = HashMap::new();
    paths[0].iter().for_each(|coordinate| {
        intersections.insert(*coordinate, 0);
    });

    paths.iter().skip(1).for_each(|path| {
        path.iter().for_each(|coordinate| {
            if intersections.contains_key(coordinate) {
                intersections.insert(*coordinate, 1);
            }
        });
    });

    // Remove the origin, it's not really an intersection.
    intersections.remove(&(0, 0));

    // println!("intersections: {:?}", intersections);
    intersections
}

fn directions_into_coordinates(directions: &Directions) -> Vec<Coordinate> {
    // Always start at the core.
    let mut coordinate = (0, 0);
    // println!("coordinate: {:?}", coordinate);

    let mut coordinates = vec![coordinate];

    for direction in directions {
        match direction.chars().next().unwrap() {
            'D' => {
                let distance = direction.trim_start_matches('D').parse().unwrap();
                // println!("{:?} => {:?}", direction, distance);
                for _ in 1..=distance {
                    coordinate.1 -= 1;
                    coordinates.push(coordinate);
                    // println!("coordinate: {:?}", coordinate);
                }
            }
            'U' => {
                let distance = direction.trim_start_matches('U').parse().unwrap();
                // println!("{:?} => {:?}", direction, distance);
                for _ in 1..=distance {
                    coordinate.1 += 1;
                    coordinates.push(coordinate);
                    // println!("coordinate: {:?}", coordinate);
                }
            }
            'R' => {
                let distance = direction.trim_start_matches('R').parse().unwrap();
                // println!("{:?} => {:?}", direction, distance);
                for _ in 1..=distance {
                    coordinate.0 += 1;
                    coordinates.push(coordinate);
                    // println!("coordinate: {:?}", coordinate);
                }
            }
            'L' => {
                let distance = direction.trim_start_matches('L').parse().unwrap();
                // println!("{:?} => {:?}", direction, distance);
                for _ in 1..=distance {
                    coordinate.0 -= 1;
                    coordinates.push(coordinate);
                    // println!("coordinate: {:?}", coordinate);
                }
            }
            _ => panic!(),
        }
    }

    coordinates
}

fn manhattan_distance(coordinate: Coordinate) -> u32 {
    (coordinate.0.abs() + coordinate.1.abs())
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test_libs {
    use super::*;

    #[test]
    fn test_directions_into_coordinates() {
        assert_eq!(
            directions_into_coordinates(&vec!(String::from("U3"))),
            vec!((0, 0), (0, 1), (0, 2), (0, 3))
        );
        assert_eq!(
            directions_into_coordinates(&vec!(String::from("U3"), String::from("R2"))),
            vec!((0, 0), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3))
        );
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0)), 0);
        assert_eq!(manhattan_distance((0, 1)), 1);
        assert_eq!(manhattan_distance((1, 0)), 1);
        assert_eq!(manhattan_distance((0, -1)), 1);
        assert_eq!(manhattan_distance((-1, 0)), 1);
        assert_eq!(manhattan_distance((1, 1)), 2);
        assert_eq!(manhattan_distance((1, -1)), 2);
        assert_eq!(manhattan_distance((-1, 1)), 2);
        assert_eq!(manhattan_distance((-1, -1)), 2);
    }
}

#[cfg(test)]
mod test_part1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part2(&mut input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_part2(&mut input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
