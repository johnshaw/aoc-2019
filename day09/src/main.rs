use std::io::{self, BufRead};
use std::rc::Rc;
use common::intcode::*;
use common::myasync::*;

async fn part_1() {
    if let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let (mut s, r) = make_channel();
        s.send(1);
        let (mut r, _) = run_async(Rc::new(line), r, s).await;
        println!("{}", r.recv().await);
    }
}

async fn part_2() {
    if let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let (mut s, r) = make_channel();
        s.send(2);
        let (mut r, _) = run_async(Rc::new(line), r, s).await;
        println!("{}", r.recv().await);
    }
}

fn main() {
    //execute_one(part_1());
    execute_one(part_2());
}
