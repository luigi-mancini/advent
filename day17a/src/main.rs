use anyhow::{Context,Result};
use regex::{Captures, Regex};

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug, Default)]
struct CPU {
    reg_a : usize,
    reg_b : usize,
    reg_c : usize,
    program: Vec<usize>,
    instr_ptr: usize,
    output: Vec<usize>,
}

impl CPU {

     fn combo_operand(&self, oper: usize) -> usize
     {
	match oper {
	    0..=3 => oper,
	    4 => self.reg_a,
    	    5 => self.reg_b,
    	    6 => self.reg_c,
	    _ => panic!("Unexpected combo operand value")
	}
     }

     fn adv(&mut self, oper: usize) -> usize {
     	self.reg_a = self.reg_a / (2 as usize).pow(self.combo_operand(oper) as u32);
     	2
     }

     fn bxl(&mut self, oper: usize) -> usize {
     	self.reg_b = self.reg_b ^ oper;
     	2
     }

     fn bst(&mut self, oper: usize) -> usize {
    	self.reg_b = self.combo_operand(oper) % 8;
     	2
     }

     fn jnz(&mut self, oper: usize) -> usize {
     	if self.reg_a == 0 {
     	   2
	} else {
	  self.instr_ptr = oper;
	  0
	}
     }

     fn bxc(&mut self, oper: usize) -> usize {
     	self.reg_b = self.reg_b ^ self.reg_c;
     	2
     }

     fn out(&mut self, oper: usize) -> usize {
     	self.output.push(self.combo_operand(oper) % 8);
     	2
     }

     fn bdv(&mut self, oper: usize) -> usize {
       	self.reg_b = self.reg_a / (2 as usize).pow(self.combo_operand(oper) as u32);
     	2
     }

     fn cdv(&mut self, oper: usize) -> usize {
       	self.reg_c = self.reg_a / (2 as usize).pow(self.combo_operand(oper) as u32);
     	2
     }

     fn get_next_instruction(&self) -> Option<(usize, usize)> {
     	if self.instr_ptr >= self.program.len() - 1 {
	    None
	} else {
	    Some((self.program[self.instr_ptr], self.program[self.instr_ptr + 1]))
	}
     }


     fn run(&mut self) {
     	while let Some((opcode, operand)) = self.get_next_instruction() {

	      self.instr_ptr += 
	      match opcode {
	      	    0 => self.adv(operand),
    	      	    1 => self.bxl(operand),
    	      	    2 => self.bst(operand),
    	      	    3 => self.jnz(operand),
    	      	    4 => self.bxc(operand),
 	      	    5 => self.out(operand),
    	      	    6 => self.bdv(operand),
    	      	    7 => self.cdv(operand),
		    _ => panic!("Unexpecred Opcode"),
	      };
	}
     }
}

fn get_num(pos: usize, mat: &Captures<'_>) -> Result<usize> {
    let x = mat.get(pos).context("regex get failed")?.as_str();
    let tmp = x.parse::<usize>().context("Parsing regex int failed")?;
    Ok(tmp)
}

fn read_input(path: &str) -> Result<CPU> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(
        r"(?x)
         (?P<m1>Register\ A:\ (\d+)) |
         (?P<m2>Register\ B:\ (\d+)) |
         (?P<m3>Register\ C:\ (\d+)) |
         (?P<m4>Program:\ (.+))",
    )?;

    let mut cpu = CPU::default();

    for l in reader.lines() {
    	let l_str = l.unwrap_or_default();

	for mat in re.captures_iter(&l_str) {
            if mat.name("m1").is_some() {
	       cpu.reg_a = get_num(2, &mat)?;
            } else if mat.name("m2").is_some() {
	      cpu.reg_b = get_num(4, &mat)?;
            } else if mat.name("m3").is_some() {
    	      cpu.reg_c = get_num(6, &mat)?;
	    } else if mat.name("m4").is_some() {
	        cpu.program = mat.get(8)
		           .context("regex get failed")?
			   .as_str()
			   .split(',')
			   .map(|x| x.parse::<usize>().context("Parsing regex int failed"))
			   .collect::<Vec<Result<usize, _>>>()
			   .into_iter().collect::<Result<Vec<_>, _>>()?;
	    }
	}
    }
    
    Ok(cpu)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut cpu = read_input("day17.txt")?;

    println!("{:?}", cpu);
    
    cpu.run();

    println!("{:?}", cpu);

    Ok(())
}
