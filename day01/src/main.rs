use std::io::*;

fn main() {
    // Part 1
    //let n: u32 = stdin().lock().lines()
    //    .map(|n| (n.unwrap().parse::<u32>().unwrap() / 3) - 2).sum();
    let n: i32 = stdin()
        .lock()
        .lines()
        .map(|n| {
            let mut n = n.unwrap().parse::<i32>().unwrap();
            n = n / 3 - 2;
            let mut total = 0;
            while n > 0 {
                total += n;
                n = n / 3 - 2;
            }
            total
        })
        .sum();
    println!("{}", n);
}
