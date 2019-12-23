use std::io::{self, BufRead};
use std::rc::Rc;
use common::intcode::*;
use common::myasync::*;

async fn part_1() {
    if let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let (mut s, mut r) = make_channel();
        s.send(1);
        let (mut r, _) = run_async(Rc::new(line), r, s).await;
        println!("{}", r.recv().await);
    }
}

fn main() {
    execute_one(part_1());
}
