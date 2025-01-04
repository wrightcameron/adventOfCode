use std::collections::HashMap;

#[derive(Debug)]
struct Instruction<'a> {
    command: Option<Command>,
    args: [&'a str; 2],
    destination: &'a str,
}

#[derive(Debug)]
enum Command {
    And,
    Or,
    LShift,
    RShift,
    Not,
}

impl <'a> Instruction<'a> {
    pub fn new(instruct_str: &'a str) -> Self {
        let (left, right) = instruct_str.split_once("->").unwrap();
        let destination = right.trim();
        let statement = left.trim().split_whitespace().collect::<Vec<&str>>();
        let mut args: [&str; 2] = [""; 2];
        let mut command: Option<Command> = None;
        // Exp: 123 -> x
        if statement.len() == 1 {
            args[0] = statement[0];
            command = None;
        } 
        // Exp: NOT x -> h
        else if statement.len() == 2 {
            args[0] = statement[1];
            command = Some(Command::Not);
        } 
        // Exp: x AND y -> d
        else if statement.len() == 3 {
            args[0] = statement[0];
            args[1] = statement[2];
            let operation = statement[1];
            command = match operation {
                "AND" => {
                    Some(Command::And)
                },
                "OR" => {
                    Some(Command::Or)
                },
                "LSHIFT" => {
                    Some(Command::LShift)
                },
                "RSHIFT" => {
                    Some(Command::RShift)
                },
                _ => panic!("Unrecognized symbol")
            };
        } else {
            panic!("Left side statement has 0 or greater than 3 elements");
        }
        Instruction {
            command,
            args,
            destination
        }
    }
}

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    parse_and_find_signal(input, "a")
}

fn parse_and_find_signal(input: &str, wire: &str) -> i32 {
    let mut instructions: HashMap<&str, Instruction> = HashMap::new();
    for instruction in input.lines().map(| line | Instruction::new(line)) {
        instructions.insert(instruction.destination, instruction);
    }
    let mut known_wires: HashMap<&str, u16> = HashMap::new();
    calculate_signal(wire, &instructions, &mut known_wires) as i32
}

fn calculate_signal<'a>(wire: &str, instructions: &HashMap<&str, Instruction<'a>>, known_wires: &mut HashMap<&'a str, u16>) -> u16 {
    let wire = instructions.get(wire).unwrap();
    // dbg!(wire);
    match wire.command {
        Some(Command::And) => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            let arg2 = if wire.args[1].chars().all(| c | c.is_digit(10)){
                wire.args[1].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[1]) {
                    *known_wires.get(wire.args[1]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[1], &instructions, known_wires);
                    known_wires.insert(wire.args[1], calc);
                    calc
                }
            };
            arg1 & arg2
        },
        Some(Command::Or) => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            let arg2 = if wire.args[1].chars().all(| c | c.is_digit(10)){
                wire.args[1].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[1]) {
                    *known_wires.get(wire.args[1]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[1], &instructions, known_wires);
                    known_wires.insert(wire.args[1], calc);
                    calc
                }
            };
            arg1 | arg2
        },
        Some(Command::LShift) => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            let arg2 = if wire.args[1].chars().all(| c | c.is_digit(10)){
                wire.args[1].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[1]) {
                    *known_wires.get(wire.args[1]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[1], &instructions, known_wires);
                    known_wires.insert(wire.args[1], calc);
                    calc
                }
            };
            arg1 << arg2
        },
        Some(Command::RShift) => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            let arg2 = if wire.args[1].chars().all(| c | c.is_digit(10)){
                wire.args[1].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[1]) {
                    *known_wires.get(wire.args[1]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[1], &instructions, known_wires);
                    known_wires.insert(wire.args[1], calc);
                    calc
                }
            };
            arg1 >> arg2
        },
        Some(Command::Not) => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            ! arg1
        },
        None => {
            let arg1 = if wire.args[0].chars().all(| c | c.is_digit(10)){
                wire.args[0].parse().unwrap()
            }else{
                if known_wires.contains_key(wire.args[0]) {
                    *known_wires.get(wire.args[0]).unwrap()
                } else {
                    let calc = calculate_signal(wire.args[0], &instructions, known_wires);
                    known_wires.insert(wire.args[0], calc);
                    calc
                }
            };
            arg1
        }
    }
}

fn return_digit_or_variable(signals: &HashMap<&str, u16>, value: &str) -> u16 {
    if value.chars().all(| c | c.is_digit(10)){
        value.parse::<u16>().unwrap()
    } else {
        if signals.contains_key(value) {
            *signals.get(value).unwrap()
        } else {
            0
        }
    }
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::test_utils::get_solution;

    #[test]
    fn test_parse_and_find_signal() {
        let input = "123 -> x\n\
                        456 -> y\n\
                        x AND y -> d\n\
                        x OR y -> e\n\
                        x LSHIFT 2 -> f\n\
                        y RSHIFT 2 -> g\n\
                        NOT x -> h\n\
                        NOT y -> i";
        let expected = 72;
        assert_eq!(parse_and_find_signal(input, "d"), expected);
        let expected = 507;
        assert_eq!(parse_and_find_signal(input, "e"), expected);
        let expected = 492;
        assert_eq!(parse_and_find_signal(input, "f"), expected);
        let expected = 114;
        assert_eq!(parse_and_find_signal(input, "g"), expected);
        let expected = 65412;
        assert_eq!(parse_and_find_signal(input, "h"), expected);
        let expected = 65079;
        assert_eq!(parse_and_find_signal(input, "i"), expected);
        let expected = 123;
        assert_eq!(parse_and_find_signal(input, "x"), expected);
        let expected = 456;
        assert_eq!(parse_and_find_signal(input, "y"), expected);
    }

    #[test]
    fn test_problem1() {
        //Actual
        let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day07".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day07".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
