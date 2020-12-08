#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    ACC,
    JMP,
    NOP,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Inst {
    op: Op,
    arg: i64,
    runcount: i64,
}

#[derive(Debug)]
struct VM {
    instructions: Vec<Inst>,
    ip: i64,
    accumulator: i64,
}

impl VM {
    fn step(&mut self) {
        let inst = self.instructions.get_mut(self.ip as usize).unwrap();
        inst.runcount += 1;

        match inst.op {
            Op::ACC => self.accumulator += inst.arg,
            Op::JMP => self.ip += inst.arg,
            Op::NOP => {}
        };

        if inst.op != Op::JMP {
            self.ip += 1;
        };
    }
}

fn parse_line(line: &str) -> Result<Inst, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    let op = match parts[0] {
        "acc" => Op::ACC,
        "jmp" => Op::JMP,
        "nop" => Op::NOP,
        _ => panic!("Didn't recognize op code"),
    };
    let arg = parts[1].parse::<i64>()?;
    Ok(Inst {
        op,
        arg,
        runcount: 0,
    })
}

fn day8a(lines: &[String]) -> i64 {
    let instructions: Vec<Inst> = lines.iter().map(|line| parse_line(line).unwrap()).collect();
    let mut vm = VM {
        instructions,
        ip: 0,
        accumulator: 0,
    };
    loop {
        if vm.instructions.get(vm.ip as usize).unwrap().runcount > 0 {
            break;
        }
        vm.step();
    }
    vm.accumulator
}

fn fix_inst(is: &[Inst], linenum: usize) -> Vec<Inst> {
    let mut newinst = is.to_vec();
    if newinst[linenum].op == Op::JMP {
        newinst[linenum].op = Op::NOP;
    } else if newinst[linenum].op == Op::NOP {
        newinst[linenum].op = Op::JMP;
    }
    newinst
}

fn day8b(lines: &[String]) -> i64 {
    let instructions: Vec<Inst> = lines.iter().map(|line| parse_line(line).unwrap()).collect();
    let instlen = lines.len();
    for fixline in 0..instlen {
        let fixed = fix_inst(&instructions, fixline);
        let mut vm = VM {
            instructions: fixed,
            ip: 0,
            accumulator: 0,
        };
        while (vm.ip as usize) < instlen {
            if vm.instructions.get(vm.ip as usize).unwrap().runcount > 0 {
                break;
            }
            vm.step();
            if (vm.ip as usize) >= instlen {
                return vm.accumulator;
            }
        }
    }
    0
}

pub fn day8(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day8a(lines),
        'b' => day8b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day8;
    #[test]
    fn test_case() {
        let test_inst = "acc +1";
        assert_eq!(
            day8::parse_line(test_inst).unwrap(),
            day8::Inst {
                op: day8::Op::ACC,
                arg: 1,
                runcount: 0
            }
        );
        let test_inst_neg = "jmp -1";
        assert_eq!(
            day8::parse_line(test_inst_neg).unwrap(),
            day8::Inst {
                op: day8::Op::JMP,
                arg: -1,
                runcount: 0
            }
        );

        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let lines: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();
        let day8a = day8::day8a(&lines);
        assert_eq!(day8a, 5);
        let day8b = day8::day8b(&lines);
        assert_eq!(day8b, 8);
    }
}
