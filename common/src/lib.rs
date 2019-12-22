pub mod myasync;
pub mod intcode;

#[cfg(test)]
mod tests {
    use super::myasync::*;
    use std::pin::Pin;

    async fn print(msg: &str) {
        println!("I am printer: {}", msg);
    }

    async fn run(mut q: Receiver<u32>) {
        let v = q.recv().await;
        println!("I got thing from queue! {}", v);
    }

    async fn foo() {
        println!("I ran an async function!");

        let (mut s, r) = make_channel();
        s.send(42u32);

        let p1 = Pin::from(Box::new(print("foo")));
        let p2 = Pin::from(Box::new(print("bar")));
        let p3 = Pin::from(Box::new(run(r)));

        wait_many(vec![p1, p2, p3]).await;

        //run(q).await;

        //q.pop().await;
        //println!("I'll never make it. :(");
    }

    #[test]
    fn it_works() {
        execute_one(foo());
    }
}
