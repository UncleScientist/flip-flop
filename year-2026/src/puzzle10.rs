use std::{convert::Infallible, fmt::Display, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-10.txt").expect("missing input file");
    let computer = Computer::new(&data.lines().collect::<Vec<&str>>());
    println!("Puzzle 10, part 1 = {}", computer.run()[0]);
    println!("Puzzle 10, part 2 = {}", computer.run_up_to_5000000());

    /*
    for (line_num, inst) in computer.instr.iter().enumerate() {
        let mut label_found = false;
        for (label_id, jump_loc) in computer.labels.iter().enumerate() {
            if line_num == *jump_loc {
                print!("L{label_id:<5} ");
                label_found = true;
                break;
            }
        }
        if !label_found {
            print!("       ");
        }
        println!("{inst}");
    }
    println!("---");
    */
    println!("Puzzle 10, part 3 = {}", computer.run_1048576());
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::Load(val, reg) => format!("load r{reg}, {val}"),
                Instruction::Copy(src, dst) => format!("move r{dst}, r{src}"),
                Instruction::Add(src1, src2, dst) => format!("add  r{dst}, r{src1}, r{src2}"),
                Instruction::Sub(src1, src2, dst) => format!("sub  r{dst}, r{src1}, r{src2}"),
                Instruction::Mul(src1, src2, dst) => format!("mul  r{dst}, r{src1}, r{src2}"),
                Instruction::Mod(src1, src2, dst) => format!("mod  r{dst}, r{src1}, r{src2}"),
                Instruction::Inc(reg) => format!("inc  r{reg}"),
                Instruction::Dec(reg) => format!("dec  r{reg}"),
                Instruction::Jump(dst) => format!("jmp  L{dst}"),
                Instruction::Jeq(reg, dst) => format!("jeq  r{reg}, L{dst}"),
                Instruction::Jne(reg, dst) => format!("jne  r{reg}, L{dst}"),
            }
        )
    }
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

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

        // instr[6] = Instruction::Load(7, Reg(15));

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

    fn run_up_to_5000000(&self) -> usize {
        let mut count = 0;

        for starting_val in 0..100 {
            let mut regs = [0u16; 16];
            regs[0] = starting_val;
            let mut pc = 0;
            let mut step_count = 0;
            while pc < self.instr.len() {
                self.step(&mut pc, &mut regs);
                step_count += 1;
                if step_count > 5_000_000 {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    fn run_1048576(&self) -> usize {
        let count = 0;
        /*
         * TODO: Fix this to actually return a value!
         *
         *
        let mut previous = Vec::new();

        for r0_start in 0..=65535 {
            // print!("{r0_start:<5}:");
            let mut val = 0;
            for r1_start in 0..16 {
                let mut regs = [0u16; 16];
                regs[0] = r0_start;
                regs[1] = r1_start;
                let mut pc = 0;
                let mut step_count = 0;
                while pc < self.instr.len() {
                    self.step(&mut pc, &mut regs);
                    step_count += 1;
                    if step_count > 5_000_000 {
                        count += 1;
                        break;
                    }
                }
                // print!(" {}", if pc < self.instr.len() { "." } else { "*" });
                val <<= 1;
                if pc >= self.instr.len() {
                    val |= 1;
                }
            }
            previous.push(val);
            if previous.len().is_multiple_of(2) {
                let half = previous.len() / 2;
                if previous[0..half] == previous[half..] {
                    println!("Possible loop, period = {half}, count = {count}:");
                    println!(" > {:?}", &previous[0..half]);
                    println!(" > {:?}", &previous[half..]);
                }
            }
        }
        */

        count
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
