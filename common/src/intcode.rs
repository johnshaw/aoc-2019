use std::collections::VecDeque;
use std::collections::HashMap;
use std::rc::Rc;
use super::myasync::{Sender, Receiver};

pub fn run(line: &str, input: &mut VecDeque<i64>) -> Vec<i64> {
    let mut code: HashMap<usize, i64> = line
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect();

    let mut output = Vec::new();
    let mut pc = 0usize; // program counter

    let access = |code: &mut HashMap<usize, i64>, mode, value| {
        match mode {
            0 => *code.entry(value as usize).or_default(),
            1 => value,
            _ => panic!("Invalid parameter mode")
        }
    };

    loop {
        let op = *code.entry(pc).or_default();
        let (mut modes, op) = (op / 100, op % 100);

        println!("{:?}", (pc..pc+4).map(|n| code[&n]).collect::<Vec<i64>>());
        println!("pc={} op={}", pc, op);

        let mut pop_mode = || {
            let m = modes % 10;
            modes /= 10;
            m
        };

        match op {
            1 => {
                let (x, y, p) = (code[&(pc+1)], code[&(pc+2)], code[&(pc+3)]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                let v = access(&mut code, m1, x) + access(&mut code, m2, y);
                code.insert(p as usize, v);
                pc += 4;
            },
            2 => {
                let (x, y, p) = (code[&(pc+1)], code[&(pc+2)], code[&(pc+3)]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                let v = access(&mut code, m1, x) * access(&mut code, m2, y);
                code.insert(p as usize, v);
                pc += 4;
            },
            3 => {
                let p = code[&(pc+1)];
                let v = input.pop_front().expect("Empty input");
                code.insert(p as usize, v);
                println!("p={} input={}", p, v);
                pc += 2;
            },
            4 => {
                let p = code[&(pc+1)];
                let m1 = pop_mode();
                println!("p={} m1={}", p, m1);
                output.push(access(&mut code, m1, p));
                pc += 2;
            },
            5 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                if p1 != 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                if p1 == 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                let p3 = code[&(pc+3)];
                if p1 < p2 {
                    code.insert(p3 as usize, 1);
                } else {
                    code.insert(p3 as usize, 0);
                }
                pc += 4;
            },
            8 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                let p3 = code[&(pc+3)];
                if p1 == p2 {
                    code.insert(p3 as usize, 1);
                } else {
                    code.insert(p3 as usize, 0);
                }
                pc += 4;
            },
            99 => break,
            _ => panic!("bad instruction")
        }
    }

    output
}

pub async fn run_async(line: Rc<String>, mut input: Receiver<i64>, mut output: Sender<i64>) 
    -> (Receiver<i64>, Sender<i64>)
{
    let mut code: HashMap<usize, i64> = line
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect();

    let mut pc = 0usize; // program counter

    let access = |code: &mut HashMap<usize, i64>, mode, value| {
        match mode {
            0 => *code.entry(value as usize).or_default(),
            1 => value,
            _ => panic!("Invalid parameter mode")
        }
    };

    loop {
        let op = *code.entry(pc).or_default();
        let (mut modes, op) = (op / 100, op % 100);

        //println!("{:?}", (pc..pc+4).map(|n| code[&n]).collect::<Vec<i64>>());
        println!("pc={} op={}", pc, op);

        let mut pop_mode = || {
            let m = modes % 10;
            modes /= 10;
            m
        };

        match op {
            1 => {
                let (x, y, p) = (code[&(pc+1)], code[&(pc+2)], code[&(pc+3)]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                let v = access(&mut code, m1, x) + access(&mut code, m2, y);
                code.insert(p as usize, v);
                pc += 4;
            },
            2 => {
                let (x, y, p) = (code[&(pc+1)], code[&(pc+2)], code[&(pc+3)]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                let v = access(&mut code, m1, x) * access(&mut code, m2, y);
                code.insert(p as usize, v);
                pc += 4;
            },
            3 => {
                let p = code[&(pc+1)];
                let v = input.recv().await;
                code.insert(p as usize, v);
                println!("p={} input={}", p, v);
                pc += 2;
            },
            4 => {
                let p = code[&(pc+1)];
                let m1 = pop_mode();
                println!("p={} m1={}", p, m1);
                output.send(access(&mut code, m1, p));
                pc += 2;
            },
            5 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                if p1 != 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                if p1 == 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                let p3 = code[&(pc+3)];
                if p1 < p2 {
                    code.insert(p3 as usize, 1);
                } else {
                    code.insert(p3 as usize, 0);
                }
                pc += 4;
            },
            8 => {
                let (arg1, arg2) = (code[&(pc+1)], code[&(pc+2)]);
                let p1 = access(&mut code, pop_mode(), arg1);
                let p2 = access(&mut code, pop_mode(), arg2);
                let p3 = code[&(pc+3)];
                if p1 == p2 {
                    code.insert(p3 as usize, 1);
                } else {
                    code.insert(p3 as usize, 0);
                }
                pc += 4;
            },
            99 => break,
            _ => panic!("bad instruction")
        }
    }

    (input, output)
}