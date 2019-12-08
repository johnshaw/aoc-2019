use std::io::*;

fn part1(line: &str, input: &[i64], output: &mut Vec<i64>) -> i64 {
    let mut nums: Vec<_> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    nums.resize(1000, 0);

    println!("nums size = {}", nums.len());

    let mut pc = 0usize; // program counter
    let mut ip = 0usize; // input pointer

    let access = |nums: &Vec<i64>, mode, value| {
        match mode {
            0 => nums[value as usize],
            1 => value,
            _ => panic!("Invalid parameter mode")
        }
    };

    loop {
        let op = nums[pc];
        let (mut modes, op) = (op / 100, op % 100);

        println!("{:?}", &nums[pc..pc+4]);
        println!("pc={} ip={} op={}", pc, ip, op);

        let mut pop_mode = || {
            let m = modes % 10;
            modes /= 10;
            m
        };

        match op {
            1 => {
                let (x, y, p) = (nums[pc+1], nums[pc+2], nums[pc+3]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                nums[p as usize] = access(&nums, m1, x) + access(&nums, m2, y);
                pc += 4;
            },
            2 => {
                let (x, y, p) = (nums[pc+1], nums[pc+2], nums[pc+3]);
                let m1 = pop_mode();
                let m2 = pop_mode();
                let m3 = pop_mode();
                println!("x={} y={} p={} m1={} m2={} m3={}", x, y, p, m1, m2, m3);
                nums[p as usize] = access(&nums, m1, x) * access(&nums, m2, y);
                pc += 4;
            },
            3 => {
                let p = nums[pc+1];
                nums[p as usize] = input[ip];
                println!("p={} input={}", p, input[ip]);
                pc += 2;
                ip += 1;
            },
            4 => {
                let p = nums[pc+1];
                let m1 = pop_mode();
                println!("p={} m1={}", p, m1);
                output.push(access(&nums, m1, p));
                pc += 2;
            },
            5 => {
                let p1 = access(&nums, pop_mode(), nums[pc+1]);
                let p2 = access(&nums, pop_mode(), nums[pc+2]);
                if p1 != 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let p1 = access(&nums, pop_mode(), nums[pc+1]);
                let p2 = access(&nums, pop_mode(), nums[pc+2]);
                if p1 == 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let p1 = access(&nums, pop_mode(), nums[pc+1]);
                let p2 = access(&nums, pop_mode(), nums[pc+2]);
                //let p3 = access(&nums, pop_mode(), nums[pc+3]);
                let p3 = nums[pc+3];
                if p1 < p2 {
                    nums[p3 as usize] = 1;
                } else {
                    nums[p3 as usize] = 0;
                }
                pc += 4;
            },
            8 => {
                let p1 = access(&nums, pop_mode(), nums[pc+1]);
                let p2 = access(&nums, pop_mode(), nums[pc+2]);
                //let p3 = access(&nums, pop_mode(), nums[pc+3]);
                let p3 = nums[pc+3];
                if p1 == p2 {
                    nums[p3 as usize] = 1;
                } else {
                    nums[p3 as usize] = 0;
                }
                pc += 4;
            },
            99 => break,
            _ => panic!("bad instruction")
        }
    }

    nums[0]
}

//fn part2(line: &str) -> i64 {
//    for noun in 0..99 {
//        for verb in 0..99 {
//            if part1(line, noun, verb) == 19690720 {
//                return noun * 100 + verb;
//            }
//        }
//    }
//    panic!("No solution");
//}

fn main() {
    let line = stdin().lock().lines().next().unwrap().unwrap();
    let mut output = Vec::new();
    println!("{}", part1(&line, &[1], &mut output));
    println!("{:?}", output);
    let mut output = Vec::new();
    println!("{}", part1(&line, &[5], &mut output));
    println!("{:?}", output);
    //println!("{}", part2(&line));
}
