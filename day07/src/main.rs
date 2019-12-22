use std::io::{self, BufRead};
use common::*;
use itertools::Itertools;
use std::pin::Pin;
use std::future::Future;
use std::rc::Rc;

async fn part_2(line: String) {
    let line = Rc::new(line);
    let mut results = Vec::new();
    for perm in (5..10).permutations(5) {
        // Create connections
        let (mut s1, r1) = myasync::make_channel();
        let (mut s2, r2) = myasync::make_channel();
        let (mut s3, r3) = myasync::make_channel();
        let (mut s4, r4) = myasync::make_channel();
        let (mut s5, r5) = myasync::make_channel();

        // Phase settings
        s1.send(perm[0]);
        s2.send(perm[1]);
        s3.send(perm[2]);
        s4.send(perm[3]);
        s5.send(perm[4]);

        // Initial input
        s5.send(0);

        // Create amps and link them together
        let ampa = Box::pin(intcode::run_async(line.clone(), r5, s1));
        let ampb = Box::pin(intcode::run_async(line.clone(), r1, s2));
        let ampc = Box::pin(intcode::run_async(line.clone(), r2, s3));
        let ampd = Box::pin(intcode::run_async(line.clone(), r3, s4));
        let ampe = Box::pin(intcode::run_async(line.clone(), r4, s5));

        let tasks: Vec<Pin<Box<dyn Future<Output=_>>>> =
            vec![ampa, ampb, ampc, ampd, ampe];

        let mut res = myasync::wait_many(tasks).await;
        println!("Amps complete");
        results.push(res[0].0.recv().await);
    }
    let m = results.into_iter().max().unwrap();
    println!("{}", m);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Part 1
    //if let Some(Ok(line)) = io::stdin().lock().lines().next() {
    //    let m = (5..10).permutations(5)
    //        .map(|perm| {
    //            perm.into_iter()
    //                .fold(0i64, |a, b| {
    //                    *intcode::run(&line, &[b, a]).last().unwrap()
    //                })
    //        })
    //        .max();

    //    println!("{:?}", m);
    //}

    // Part 2
    if let Some(Ok(line)) = io::stdin().lock().lines().next() {
        myasync::execute_one(part_2(line));
    }

    Ok(())
}