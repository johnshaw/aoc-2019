use std::io::*;

fn part1(line: &str, noun: usize, verb: usize) -> usize {
    let mut nums: Vec<_> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    nums[1] = noun;
    nums[2] = verb;

    for idx in (0..nums.len() - 4).step_by(4) {
        let w = &nums[idx..idx+4];
        let (op, x, y, pos) = (w[0], w[1], w[2], w[3]);
        nums[pos] = match op {
            1 => nums[x] + nums[y],
            2 => nums[x] * nums[y],
            99 => break,
            _ => panic!("uh oh")
        }
    }

    nums[0]
}

fn part2(line: &str) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            if part1(line, noun, verb) == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    panic!("No solution");
}

fn main() {
    let line = stdin().lock().lines().next().unwrap().unwrap();
    //println!("{}", part1(&line, 12, 2));
    println!("{}", part2(&line));
}
