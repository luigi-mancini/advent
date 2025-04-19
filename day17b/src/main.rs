use anyhow::{Context, Result};
use regex::{Captures, Regex};

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug, Default)]
struct Cpu {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
    instr_ptr: usize,
    output: Vec<usize>,
    reg_a_val: usize,
}

impl Cpu {
    fn combo_operand(&self, oper: usize) -> usize {
        match oper {
            0..=3 => oper,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unexpected combo operand value"),
        }
    }

    fn adv(&mut self, oper: usize) {
        self.reg_a /= (2_usize).pow(self.combo_operand(oper) as u32);
        self.instr_ptr += 2;
    }

    fn bxl(&mut self, oper: usize) {
        self.reg_b ^= oper;
        self.instr_ptr += 2;
    }

    fn bst(&mut self, oper: usize) {
        self.reg_b = self.combo_operand(oper) % 8;
        self.instr_ptr += 2;
    }

    #[allow(dead_code)]
    fn jnz(&mut self, oper: usize) {
        if self.reg_a == 0 {
            self.instr_ptr += 2;
        } else {
            self.instr_ptr = oper;
        }
    }

    fn bxc(&mut self, _oper: usize) {
        self.reg_b ^= self.reg_c;
        self.instr_ptr += 2;
    }

    fn out(&mut self, oper: usize) {
        self.output.push(self.combo_operand(oper) % 8);
        self.instr_ptr += 2;
    }

    fn bdv(&mut self, oper: usize) {
        self.reg_b = self.reg_a / (2 as usize).pow(self.combo_operand(oper) as u32);
        self.instr_ptr += 2;
    }

    fn cdv(&mut self, oper: usize) {
        self.reg_c = self.reg_a / (2 as usize).pow(self.combo_operand(oper) as u32);
        self.instr_ptr += 2;
    }

    fn reset(&mut self, val: usize) {
        //println!("RESET");
        self.output.clear();
        self.reg_a_val = val;
        self.reg_a = self.reg_a_val;
        self.reg_b = 0;
        self.reg_c = 0;
        self.instr_ptr = 0;
    }

    fn get_next_instruction(&self) -> Option<(usize, usize)> {
        if self.instr_ptr >= self.program.len() - 1 {
            None
        } else {
            Some((
                self.program[self.instr_ptr],
                self.program[self.instr_ptr + 1],
            ))
        }
    }

    fn calc_val(&mut self, input: usize) -> usize {
        self.reset(input);

        while let Some((opcode, operand)) = self.get_next_instruction() {
            //println!("INSTR PTR {}", self.instr_ptr);

            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => break,
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Unexpecred Opcode"),
            };
        }

        self.output[0]
    }

    fn run(&mut self) -> Option<usize> {
        let plen = self.program.len();

        let mut result: usize = 0;
        let mut index_vec = vec![0; plen];
        let mut current_index = plen - 1;

        loop {
            if current_index >= plen {
                return None;
            }

            if index_vec[current_index] >= 8 {
                index_vec[current_index] = 0;
                current_index += 1;
                result >>= 3;
                continue;
            }

            let val = self.program[current_index];
            let input = (result << 3) + index_vec[current_index];

            let ret = self.calc_val(input);

            index_vec[current_index] += 1;

            if ret == val {
                result = input;
                if current_index == 0 {
                    break;
                }
                current_index -= 1;
            }
        }
        Some(result)
    }
}

fn get_num(pos: usize, mat: &Captures<'_>) -> Result<usize> {
    let x = mat.get(pos).context("regex get failed")?.as_str();
    let tmp = x.parse::<usize>().context("Parsing regex int failed")?;
    Ok(tmp)
}

fn read_input(path: &str) -> Result<Cpu> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(
        r"(?x)
         (?P<m1>Register\ A:\ (\d+)) |
         (?P<m2>Register\ B:\ (\d+)) |
         (?P<m3>Register\ C:\ (\d+)) |
         (?P<m4>Program:\ (.+))",
    )?;

    let mut cpu = Cpu::default();

    for l in reader.lines() {
        let l_str = l.unwrap_or_default();

        for mat in re.captures_iter(&l_str) {
            if mat.name("m2").is_some() {
                cpu.reg_b = get_num(4, &mat)?;
            } else if mat.name("m3").is_some() {
                cpu.reg_c = get_num(6, &mat)?;
            } else if mat.name("m4").is_some() {
                cpu.program = mat
                    .get(8)
                    .context("regex get failed")?
                    .as_str()
                    .split(',')
                    .map(|x| x.parse::<usize>().context("Parsing regex int failed"))
                    .collect::<Vec<Result<usize, _>>>()
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
            }
        }
    }

    Ok(cpu)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut cpu = read_input("day17.txt")?;

    //println!("{:?}", cpu);

    let val = cpu.run().expect("Value calculation failed");

    let end = start.elapsed();
    println!("{:?} {:?}", end, val);

    Ok(())
}
