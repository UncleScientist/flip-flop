use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-10.txt").expect("missing input file");
    let computer = Computer::new(&data.lines().collect::<Vec<&str>>());
    println!("Puzzle 10, part 1 = {}", computer.run()[0]);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Load(u16, Reg),
    Copy(Reg, Reg),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    Mul(Reg, Reg, Reg),
    Mod(Reg, Reg, Reg),
    Inc(Reg),
    Dec(Reg),
    Jump(usize),
    Jeq(Reg, usize),
    Jne(Reg, usize),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = &line[2..].split("ne").collect::<Vec<_>>();
        let nums = words.iter().map(|word| word.len() / 2).collect::<Vec<_>>();
        Ok(match nums[0] {
            0 => Self::Load(nums[1] as u16, Reg(nums[2])),
            1 => Self::Copy(Reg(nums[1]), Reg(nums[2])),
            2 => Self::Add(Reg(nums[1]), Reg(nums[2]), Reg(nums[3])),
            3 => Self::Sub(Reg(nums[1]), Reg(nums[2]), Reg(nums[3])),
            4 => Self::Mul(Reg(nums[1]), Reg(nums[2]), Reg(nums[3])),
            5 => Self::Mod(Reg(nums[1]), Reg(nums[2]), Reg(nums[3])),
            6 => Self::Inc(Reg(nums[1])),
            7 => Self::Dec(Reg(nums[1])),
            8 => Self::Jump(nums[1]),
            9 => Self::Jeq(Reg(nums[1]), nums[2]),
            10 => Self::Jne(Reg(nums[1]), nums[2]),

            _ => panic!("Cannot interpret instruction {}", nums[0]),
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Reg(usize);

struct Computer {
    labels: Vec<usize>,
    instr: Vec<Instruction>,
}

impl Computer {
    fn new(program: &[&str]) -> Self {
        let mut labels: Vec<usize> = Vec::new();
        let mut instr: Vec<Instruction> = Vec::new();

        for line in program {
            if line.starts_with("ba") {
                instr.push(line.parse().unwrap());
            } else {
                assert!(line.starts_with("be"));
                let label_id = (line.len() - 2) / 2;
                if labels.len() <= label_id {
                    labels.resize(label_id + 1, 0);
                }
                labels[label_id] = instr.len();
            }
        }

        Self { labels, instr }
    }

    fn step(&self, pc: &mut usize, regs: &mut [u16; 16]) {
        *pc += 1;
        match self.instr[*pc - 1] {
            Instruction::Load(value, Reg(reg)) => regs[reg] = value,
            Instruction::Copy(Reg(src), Reg(dst)) => regs[dst] = regs[src],
            Instruction::Add(Reg(src1), Reg(src2), Reg(dst)) => {
                regs[dst] = regs[src1].wrapping_add(regs[src2]);
            }
            Instruction::Sub(Reg(src1), Reg(src2), Reg(dst)) => {
                regs[dst] = regs[src1].wrapping_sub(regs[src2]);
            }
            Instruction::Mul(Reg(src1), Reg(src2), Reg(dst)) => {
                regs[dst] = regs[src1].wrapping_mul(regs[src2]);
            }
            Instruction::Mod(Reg(src1), Reg(src2), Reg(dst)) => {
                regs[dst] = if regs[src2] == 0 {
                    0
                } else {
                    regs[src1] % regs[src2]
                };
            }
            Instruction::Inc(Reg(reg)) => regs[reg] = regs[reg].wrapping_add(1),
            Instruction::Dec(Reg(reg)) => regs[reg] = regs[reg].wrapping_sub(1),
            Instruction::Jump(label) => *pc = self.labels[label],
            Instruction::Jeq(Reg(reg), label) => {
                if regs[reg] == 0 {
                    *pc = self.labels[label];
                }
            }
            Instruction::Jne(Reg(reg), label) => {
                if regs[reg] != 0 {
                    *pc = self.labels[label];
                }
            }
        }
    }

    fn run(&self) -> [u16; 16] {
        let mut regs = [0u16; 16];
        let mut pc = 0;

        while pc < self.instr.len() {
            self.step(&mut pc, &mut regs);
        }

        regs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_1() {
        let program = [
            "banenanena",
            "banenananenana",
            "banenanananenanana",
            "banananenanenananenananana",
            "banananenananananenanananenanananana",
            "bananenananananane",
        ];
        let computer = Computer::new(&program);

        let result = computer.run();
        assert_eq!([6, 1, 2, 3, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], result);
    }

    #[test]
    fn test_program_2() {
        let program = [
            "banenane",
            "banenananananananananananena",
            "banenanananananananananananananananananananananananananananananananananenana",
            "banenanananananananananananananenanana",
            "banenananananananananananananananananananananananananananananananananananananananenananana",
            "banananenenananane",
            "bananananenenanane",
            "banananananenenanananane",
            "bananananananenenane",
            "banananananananane",
            "banananananananane",
            "bananananananane",
            "banananananananena",
            "banananananananenana",
            "banananananananenanana",
            "banananananananenananana",
        ];
        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!([3, 11, 34, 14, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], result);
    }

    #[test]
    fn test_program_3() {
        let program = [
            "banenanane",
            "banananananenenena",
            "banananananenanenanenana",
            "banananananenananenananenanana",
            "banananenanananenanananenananana",
            "banananenananananenananananenanananana",
            "banananenanananananenanananananenananananana",
            "banananenananananananenananananananenanananananana",
            "banananenanananananananenanananananananenananananananana",
            "banananenananananananananenananananananananenanananananananana",
            "banananenanananananananananenanananananananananenananananananananana",
        ];

        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!(
            [
                2, 4, 16, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 0, 0, 0, 0, 0
            ],
            result
        );
    }

    #[test]
    fn test_program_4() {
        let program = [
            "banenane",
            "banananananananananenananananananana",
            "banenanena",
            "be",
            "banenanenana",
            "banananananananananenanana",
            "benana",
            "banenanenanana",
            "banananananananananena",
            "benanana",
            "banenanenananana",
            "banananananananananenana",
            "benananananananananana",
            "banenanenanananana",
            "bananananananananane",
            "benananananananananananana",
            "banenanenananananana",
            "benanananananananana",
            "banenanenanananananana",
            "benananananananana",
            "banenanenananananananana",
            "banananananananananenananananananananana",
            "bena",
            "banenanenanananananananana",
        ];

        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!([1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0], result);
    }

    #[test]
    fn test_program_5() {
        let program = [
            "banenananenana",
            "banenananananananananananenanananananananananananananana",
            "be",
            "banananenanenananena",
            "banananananananenanana",
            "bananananananananenananana",
            "bananananananananenanananananananananananananana",
            "banananananananananananenananananananananananananananane",
        ];
        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!(
            [0, 20, 2, 10, 65526, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result
        );
    }

    #[test]
    fn test_program_6() {
        let program = [
            "banenananananananananananananananananananananena",
            "benanana",
            "bananananananananena",
            "banenananananananananananananananenana",
            "benananana",
            "bananananananananenana",
            "banenananananananananananenanana",
            "bena",
            "bananananananananenanana",
            "banananenenane",
            "banananenananene",
            "banananananananananananenanananena",
            "banananananananananananenananenananana",
            "banananananananananananenanenanana",
            "banenanena",
            "bananananananananane",
            "banene",
            "be",
            "bananananananananena",
            "bananananananane",
            "bananananananananananenane",
        ];
        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!(
            [44802, 65535, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result
        );
    }

    #[test]
    fn test_program_7() {
        let program = [
            "banenanena",
            "banenanananananananananananananananenanana",
            "be",
            "banananenenanenana",
            "bananenane",
            "bananenananena",
            "bananananananananenanana",
            "banananananananananananenananane",
            "banenena",
            "banenenana",
        ];
        let computer = Computer::new(&program);
        let result = computer.run();
        assert_eq!([610, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], result);
    }
}
