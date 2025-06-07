use anyhow::{Ok, Result};
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Wires {
    wires: HashMap<String, u32>,
}

impl Wires {
    fn new() -> Self {
        Wires {
            wires: HashMap::new(),
        }
    }

    fn set_value(&mut self, key: String, value: u32) {
        self.wires.insert(key, value);
    }

    fn get_value(&self, key: &str) -> Option<u32> {
        self.wires.get(key).cloned()
    }
}

#[derive(Debug, Clone)]
struct LogicGate {
    operation: Operation,
    left: String,
    right: String,
    output: String,
}

impl LogicGate {
    fn new(op: Operation, left: String, right: String, output: String) -> Self {
        LogicGate {
            operation: op,
            left,
            right,
            output,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: Wires,
    gates: HashMap<String, LogicGate>,
}

impl Circuit {

    fn new() -> Self {
        Circuit {
            wires: Wires::new(),
            gates: HashMap::new(),
        }
    }

    fn add_wire(&mut self, key: String, value: u32) {
        self.wires.set_value(key, value);
    }

    fn add_gate(&mut self, gate: LogicGate) {
        self.gates.insert(gate.output.clone(), gate);
    }
    
    fn evaluate(&mut self) -> Result<()> {

        loop {

            let gate_copy = self.gates.clone();

            for (key, gate) in gate_copy{

                if let (Some(left), Some(right)) = (self.wires.get_value(&gate.left), self.wires.get_value(&gate.right)) {
                    let result = match gate.operation {
                        Operation::And => left & right,
                        Operation::Or => left  | right,
                        Operation::Xor => left ^ right,
                    };

                    self.add_wire(gate.output, result);
                    self.gates.remove(&key);
                }             
            }

            if  self.gates.is_empty() {
                break;
            }
        }
        Ok(())
    }

    fn get_result(&self) -> u64 {
        use std::cmp::Reverse;
        let mut result = 0;


        let mut tmp : Vec<_> = self.wires.wires.iter().filter(|tmp|  tmp.0.chars().next() == Some('z')).collect();
        tmp.sort_by_key(|(k, _)| Reverse((*k).clone()));



        for (_, v) in tmp  {
            result <<= 1;
            result |= *v as u64;
            //println!("Adding {}  {:b}", v, result);
        }


        result
    }

    fn find_gate_output(&self, left: &str, right: &str, oper: Operation) -> Option<String> {

        let out = self.gates.iter().find(|(_, gate)| {
            gate.operation == oper && (
                (gate.left == left && gate.right == right) ||
                (gate.left == right && gate.right == left)) });
        
        match out {
            Some((gate_out, _)) => Some(gate_out.clone()),
            None => None,
        }
    }

    fn half_adder(&self, index: &str) -> Option<String> {
        let x: &str = &format!("x{}", index);
        let y: &str = &format!("y{}", index);  
        let z: &str = &format!("z{}", index);

         if let Some(xor) = self.find_gate_output(x, y, Operation::Xor) {
            println!("Half adder1 for {} and {}", x, y);
        
            if let Some(and) = self.find_gate_output(x, y, Operation::And) {
                if xor == z {
                    return Some(and);
                }
            }
        }
        None
    }

    fn full_adder(&self, index: &str, cin: &str) -> Option<String> {
        let x: &str = &format!("x{}", index);
        let y: &str = &format!("y{}", index);  
        let z: &str = &format!("z{}", index);

        if let Some(xor1) = self.find_gate_output(x, y, Operation::Xor) {

            if let Some(xor2) = self.find_gate_output(xor1.as_str(), cin, Operation::Xor) {
                if xor2 != z {
                    println!("full_adder xor2 is not correct output. xor1={} cin={} xor2={} out={}", xor1, cin, xor2, z);
                    return None;
                }
            } else {
                println!("full_adder xor2 not found for {} and {}", xor1, cin);
                return None;
            }

            if let Some(and1) = self.find_gate_output(xor1.as_str(), cin, Operation::And) {
                if let Some(and2) = self.find_gate_output(x, y, Operation::And) {
                    if let Some(or) = self.find_gate_output(and1.as_str(), and2.as_str(), Operation::Or) {
                        return Some(or);
                    } else {
                        println!("full_adder or not found for {} and {}", and1, and2);
                    }
                } else {
                    println!("full_adder and2 not found for {} and {}", x, y);
                }
            } else {
                println!("full_adder and1 not found for {} and {}", xor1, cin);
            }
        }   
        None
    }
}

fn read_input(path: &str) -> Result<Circuit> {
    use regex::Regex;
    use std::io::BufRead;

    let mut circuit = Circuit::new();

    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    
    let re = Regex::new(
        r"(?x)
        ^
        (?:
            (?P<key>\w+):\s*(?P<val>\d+)
          |
            (?P<left>\w+)\s+(?P<op>AND|OR|XOR)\s+(?P<right>\w+)\s*->\s*(?P<out>\w+)
        )
        $"
    ).unwrap();

    for line in reader.lines() {
        if let Some(caps) = re.captures(&line?) {
            if let (Some(k), Some(v)) = (caps.name("key"), caps.name("val")) {

                circuit.add_wire(k.as_str().to_string(), v.as_str().parse::<u32>()?);
            } else if let (Some(left), Some(op), Some(right), Some(out)) = 
                (caps.name("left"), caps.name("op"), caps.name("right"), caps.name("out")) {
    
                circuit.add_gate(LogicGate::new(
                    match op.as_str() {
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        "XOR" => Operation::Xor,
                        _ => panic!("Unknown operation"),
                    },
                    left.as_str().to_string(),
                    right.as_str().to_string(),
                    out.as_str().to_string(),
                ));
            }
        }
    }

    Ok(circuit)
}


fn main() -> Result<()> {
    let circuit = read_input("day24.txt")?;

    if let Some(cout) = circuit.half_adder("00") {

        let mut cin = cout.clone();

        for i in 1..=44 {
            let idx = format!("{:02}", i);
            let cout1 = circuit.full_adder(&idx, &cin);

            match cout1 {
                Some(c) => cin = c,
                None => {
                    println!("Full adder for {} failed", idx);
                    break;
                }
            }
            println!("Full adder output for {}: {:?}", idx, cin);
        }
    } else {
        println!("Half adder not found");
    }

    Ok(())
}