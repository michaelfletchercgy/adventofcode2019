const INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,6,19,23,2,23,6,27,1,5,27,31,1,31,9,35,2,10,35,39,1,5,39,43,2,43,10,47,1,47,6,51,2,51,6,55,2,55,13,59,2,6,59,63,1,63,5,67,1,6,67,71,2,71,9,75,1,6,75,79,2,13,79,83,1,9,83,87,1,87,13,91,2,91,10,95,1,6,95,99,1,99,13,103,1,13,103,107,2,107,10,111,1,9,111,115,1,115,10,119,1,5,119,123,1,6,123,127,1,10,127,131,1,2,131,135,1,135,10,0,99,2,14,0,0";

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut memory = Vec::new();
    parse_line(INPUT, &mut memory);
    memory[1] = 12;
    memory[2] = 2;
    
    run(&mut memory);

    println!("memory[0]={}", memory[0]);
}

fn part2() {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = Vec::new();
            parse_line(INPUT, &mut memory);
            memory[1] = noun;
            memory[2] = verb;
            run(&mut memory);

            if memory[0] == 19690720 {
                println!("noun {}, verb {}", noun, verb);
                println!("100 * noun + verb = {}", (100 * noun + verb));
            }
        }
    }
}

fn run(memory: &mut Vec<usize>) {
    let mut pc = 0;

    loop {
        match memory[pc] {
            1 => {
                let arg1 = memory[pc + 1];
                let value1 = memory[arg1];
                
                let arg2 = memory[pc + 2];
                let value2 = memory[arg2];
                
                let target = memory[pc + 3];

                //println!("arg1={}, value1={}, arg2={}, value2={}, target={}", arg1, value1, arg2, value2, target);
                memory[target] = value1 + value2;
                pc += 4;
            },
            2 => {
                let arg1 = memory[pc + 1];
                let value1 = memory[arg1];
                
                let arg2 = memory[pc + 2];
                let value2 = memory[arg2];
                
                let target = memory[pc + 3];

                //println!("arg1={}, value1={}, arg2={}, value2={}, target={}", arg1, value1, arg2, value2, target);
                memory[target] = value1 * value2;
                pc += 4;
            },
            99 => {
                break;
            }
            opcode => {
                panic!("unexpected op code {} at pc {}", opcode, pc);
            }
        }
    }
}

fn parse_line(s:&str, memory: &mut Vec<usize>) {
    let mut input:Vec<usize> = s.split(',').map(|x| x.parse().unwrap()).collect();

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
    pub fn test_sample0() {
        let mut memory = Vec::new();
        parse_line("1,0,0,3,99", &mut memory);

        run(&mut memory);

        assert_eq!(2, memory[3]);
    }

    #[test]
    pub fn test_sample1() {
        let mut memory = Vec::new();
        parse_line("1,9,10,3,2,3,11,0,99,30,40,50", &mut memory);

        run(&mut memory);

        assert_eq!(70, memory[3]);
        assert_eq!(3500, memory[0]);
    }

    #[test]
    pub fn test_final() {

        // this is wrong: 8017076
    }
}
