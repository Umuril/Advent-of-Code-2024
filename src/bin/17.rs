use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{u64, u8},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use num_traits::*;

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct CpuState {
    regs: [u64; 3],
}

#[derive(Clone, Debug, num_derive::FromPrimitive)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Clone, Debug)]
enum Operand {
    Literal(u8),
    Register(usize),
    _Reserved,
}

impl num_traits::FromPrimitive for Operand {
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u64(n as u64)
    }
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0..=3 => Some(Self::Literal(n as u8)),
            4..=6 => Some(Self::Register(n as usize - 4)),
            _ => None,
        }
    }
}

type Commands = Vec<(u8, u8)>;

fn parse_input(input: &str) -> IResult<&str, (CpuState, Commands)> {
    let (input, _) = tag("Register A: ")(input)?;
    let (input, a) = u64(input)?;
    let (input, _) = tag("\nRegister B: ")(input)?;
    let (input, b) = u64(input)?;
    let (input, _) = tag("\nRegister C: ")(input)?;
    let (input, c) = u64(input)?;

    let (input, _) = tag("\n\nProgram: ")(input)?;
    let (input, cmds) = separated_list1(tag(","), separated_pair(u8, tag(","), u8))(input)?;

    Ok((input, (CpuState { regs: [a, b, c] }, cmds)))
}

fn execute_machine(mut state: CpuState, cmds: &[(u8, u8)]) -> Vec<u8> {
    let mut ip = 0i32;
    let mut output = Vec::new();

    loop {
        if cmds.get(ip as usize).is_none() {
            break;
        }

        let (opcode_num, operand_num) = cmds[ip as usize];
        let opcode = OpCode::from_u8(opcode_num).expect("Valid opcode");
        let operand = Operand::from_u8(operand_num).expect("Valid operand");

        let combo = match operand {
            Operand::Literal(n) => n as u64,
            Operand::Register(n) => state.regs[n],
            _ => unimplemented!(),
        };

        // println!("BEFORE [{ip}] {opcode_num:?} {operand_num:?} Combo: [{combo}] {state:?} {output:?}");

        match opcode {
            OpCode::Adv => {
                state.regs[0] >>= combo;
            }
            OpCode::Bxl => state.regs[1] ^= operand_num as u64,
            OpCode::Bst => {
                state.regs[1] = combo % 8;
            }
            OpCode::Jnz => match (state.regs[0], operand) {
                (0, _) => (),
                (_, Operand::Literal(_)) => {
                    ip = (operand_num >> 1) as i32;
                }
                _ => unimplemented!(),
            },
            OpCode::Bxc => state.regs[1] ^= state.regs[2],
            OpCode::Out => {
                output.push(combo as u8 % 8);
            }
            OpCode::Bdv => {
                state.regs[1] = state.regs[0] >> combo;
            }
            OpCode::Cdv => {
                state.regs[2] = state.regs[0] >> combo;
            }
        }

        match (opcode, state.regs[0]) {
            (OpCode::Jnz, 0) => ip += 1,
            (OpCode::Jnz, _) => {}
            _ => ip += 1,
        }

        // println!("AFTER  [{ip}] {opcode_num:?} {operand_num:?} Combo: [{combo}] {state:?} {output:?}\n");
    }

    output
}

pub fn part_one(input: &str) -> Option<String> {
    let (state, cmds) = parse_input(input.trim()).expect("Correct input format").1;

    let output = execute_machine(state, &cmds);

    Some(output.iter().join(",").to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut state, cmds) = parse_input(input.trim()).expect("Correct input format").1;

    let flat_cmd: Vec<u8> = cmds
        .iter()
        .flat_map(|(a, b)| [[*a], [*b]])
        .flatten()
        .collect();
    let mut solution = 8u64.pow(flat_cmd.len() as u32 - 1);
    let mut new_inc = 1;
    let mut inc = 1;
    loop {
        // println!("SOLUTION: {solution}");
        state.regs[0] = solution;

        let output = execute_machine(state.clone(), &cmds);

        if output.len() == flat_cmd.len() {
            if output[..14] == flat_cmd[..14] {
                new_inc = 8u64.pow(12);
            }
            if output[..12] == flat_cmd[..12] {
                new_inc = 8u64.pow(10);
            } else if output[..10] == flat_cmd[..10] {
                new_inc = 8u64.pow(8);
            } else if output[..8] == flat_cmd[..8] {
                new_inc = 8u64.pow(6);
            } else if output[..6] == flat_cmd[..6] {
                new_inc = 8u64.pow(4);
            } else if output[..4] == flat_cmd[..4] {
                new_inc = 8u64.pow(2);
            }

            if new_inc > inc {
                inc = new_inc;
            }
        }

        if output == flat_cmd {
            break;
        }

        solution += inc;
    }

    Some(solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("117440".to_string()));
    }
}

/*
2 4 b = a % 8
1 1 a ^= 1
7 5 c = a >> b
0 3 a = a >> 3
1 4 a ^= 4
4 0 b ^= c
5 5 out(b)
3 0 jmp 0

b = a % 8
a ^= 1
c = a >> b
a = a >> 3
a ^= 4
b ^= c
out(b)
jmp 0

a = 1111101100100010011000010
b = 010
a = 1111101100100010011000011
c = 11111011001000100110000
a = 1111101100100010011000
a = 1111101100100010011100
b = 1111101100100010011010
out(010)


a = 0001111101100100010011100
b = 0001111101100100010011010
c = 0011111011001000100110000

*/
