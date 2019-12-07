const INPUT: &str = "3,225,1,225,6,6,1100,1,238,225,104,0,1101,69,55,225,1001,144,76,224,101,-139,224,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,1102,60,49,225,1102,51,78,225,1101,82,33,224,1001,224,-115,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,1102,69,5,225,2,39,13,224,1001,224,-4140,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,101,42,44,224,101,-120,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,68,49,224,101,-3332,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,50,27,225,1102,5,63,225,1002,139,75,224,1001,224,-3750,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,102,79,213,224,1001,224,-2844,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,1,217,69,224,1001,224,-95,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,1102,36,37,225,1101,26,16,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,1001,223,1,223,107,226,226,224,1002,223,2,223,1006,224,359,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,389,1001,223,1,223,1008,677,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,419,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,434,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,449,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,479,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,107,677,677,224,1002,223,2,223,1006,224,509,101,1,223,223,7,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,8,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,614,1001,223,1,223,1108,677,226,224,1002,223,2,223,1005,224,629,1001,223,1,223,1007,677,677,224,102,2,223,223,1006,224,644,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,659,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut memory = Vec::new();
    parse_line(INPUT, &mut memory);
    run(&mut memory);

    println!("done");
}

fn part2() {
    // for noun in 0..100 {
    //     for verb in 0..100 {
    //         let mut memory = Vec::new();
    //         parse_line(INPUT, &mut memory);
    //         memory[1] = noun;
    //         memory[2] = verb;
    //         run(&mut memory);

    //         if memory[0] == 19690720 {
    //             println!("noun {}, verb {}", noun, verb);
    //             println!("100 * noun + verb = {}", (100 * noun + verb));
    //         }
    //     }
    // }
}

#[derive(Debug)]
enum Opcode {
    Halt,
    Multiply(Vec<i64>),
    Add(Vec<i64>),
    Input,
    Output(Vec<i64>),
}

fn parse_opcode(opcode:i64) -> Opcode {
    if opcode == 99 {
        return Opcode::Halt
    };

    let mut params = Vec::new();
    let operation = opcode - (opcode / 100) * 100;

    let mut decode = opcode / 100;
    while decode > 0 {
        let param = decode - (decode / 10) * 10;
        params.push(param);

        decode = decode / 10;
    }

    match operation {
        1 => Opcode::Add(params),
        2 => Opcode::Multiply(params),
        3 => Opcode::Input,
        4 => Opcode::Output(params),
        _ => panic!("unexpected operatation {} in opcode {}", operation, opcode)
    }
}

fn get_argument(parameter_mode: i64, pos: usize, memory:&Vec<i64>) -> i64 {
    match parameter_mode {
        0 => memory[memory[pos] as usize],
        1 => memory[pos],
        x => panic!("Unexpected parameter mode {}", x)
    }
}

fn run(memory: &mut Vec<i64>) {
    let mut pc = 0;

    loop {
        match parse_opcode(memory[pc]) {
            // Add
            Opcode::Add(params) => {
                let value1 = get_argument(*params.get(0).unwrap_or(&0), pc + 1, &memory);
                let value2 = get_argument(*params.get(1).unwrap_or(&0), pc + 2, &memory);                
                let target = memory[pc + 3];

                memory[target as usize] = value1 + value2;
                pc += 4;
            },
            // Multiply
            Opcode::Multiply(params) => {
                let value1 = get_argument(*params.get(0).unwrap_or(&0), pc + 1, &memory);
                let value2 = get_argument(*params.get(1).unwrap_or(&0), pc + 2, &memory);                
                
                let target = memory[pc + 3];

                memory[target as usize] = value1 * value2;
                pc += 4;
            },
            // Input
            Opcode::Input => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                let input:i64 = buf.trim().parse().unwrap();
                let target = memory[pc + 1];
                memory[target as usize] = input;
                pc += 2;
            },
            // Output
            Opcode::Output(params) => {
                let value1 = get_argument(*params.get(0).unwrap_or(&0), pc + 1, &memory);
                println!("pc={}, output={}", pc, value1);
                pc += 2;
            },
            // Halt
            Opcode::Halt => {
                break;
            }
        }
    }
}

fn parse_line(s:&str, memory: &mut Vec<i64>) {
    let mut input:Vec<i64> = s.split(',').map(|x| x.parse().unwrap()).collect();

    memory.append(&mut input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_line() {
        let mut memory = Vec::new();
        parse_line("1,2,3", &mut memory);

        assert_eq!(vec![1,2,3], memory);
    }

    #[test]
    pub fn test_decode() {
        let decode = 1111;

        let param = decode - (decode / 10) * 10;
        
        assert_eq!(1, param);

        match parse_opcode(1002){
            Opcode::Multiply(params) => {
                assert_eq!(&0, params.get(0).unwrap_or(&0));
                assert_eq!(&1, params.get(1).unwrap_or(&0));
                assert_eq!(&0, params.get(2).unwrap_or(&0));
            },
            x => panic!("not {:?}", x)
        }
    }

    #[test]
    pub fn test_decode_test_day2_sample0() {
        match parse_opcode(1){
            Opcode::Add(params) => {
                assert_eq!(&0, params.get(0).unwrap_or(&0));
                assert_eq!(&0, params.get(1).unwrap_or(&0));
                assert_eq!(&0, params.get(2).unwrap_or(&0));
            },
            x => panic!("not {:?}", x)
        }
    }

    #[test]
    pub fn test_day2_sample0() {
        let mut memory = Vec::new();
        parse_line("1,0,0,3,99", &mut memory);

        run(&mut memory);

        assert_eq!(2, memory[3]);
    }

    #[test]
    pub fn test_day2_sample1() {
        let mut memory = Vec::new();
        parse_line("1,9,10,3,2,3,11,0,99,30,40,50", &mut memory);

        run(&mut memory);

        assert_eq!(70, memory[3]);
        assert_eq!(3500, memory[0]);
    }

    #[test]
    pub fn test_day2_part1() {
        const INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,6,19,23,2,23,6,27,1,5,27,31,1,31,9,35,2,10,35,39,1,5,39,43,2,43,10,47,1,47,6,51,2,51,6,55,2,55,13,59,2,6,59,63,1,63,5,67,1,6,67,71,2,71,9,75,1,6,75,79,2,13,79,83,1,9,83,87,1,87,13,91,2,91,10,95,1,6,95,99,1,99,13,103,1,13,103,107,2,107,10,111,1,9,111,115,1,115,10,119,1,5,119,123,1,6,123,127,1,10,127,131,1,2,131,135,1,135,10,0,99,2,14,0,0";
        let mut memory = Vec::new();
        parse_line(INPUT, &mut memory);
        memory[1] = 12;
        memory[2] = 2;
        run(&mut memory);
        assert_eq!(8017076, memory[0]);
        // this is wrong: 8017076
    }

    #[test]
    pub fn sample0() {
        const INPUT: &str = "1002,4,3,4,33";
        let mut memory = Vec::new();
        parse_line(INPUT, &mut memory);
        run(&mut memory);
        assert_eq!(99, memory[4]);
    }

    #[test]
    pub fn sample1() {
        const INPUT: &str = "1101,100,-1,4,0";
        let mut memory = Vec::new();
        parse_line(INPUT, &mut memory);
        run(&mut memory);
        assert_eq!(99, memory[4]);
    }
}
