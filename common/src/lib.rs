pub mod intcode {

use std::collections::VecDeque;
use std::collections::HashMap;

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

}

pub mod myasync {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::future::*;
    use std::pin::Pin;
    use std::task::*;

    struct AsyncQueueStuff<T> {
        queue: VecDeque<T>,
        // Tasks waiting on data from this queue
        waker: Option<Waker>
    }

    pub struct AsyncQueue<T> {
        stuff: Rc<RefCell<AsyncQueueStuff<T>>>
    }

    impl<T> AsyncQueue<T> {
        pub fn new() -> AsyncQueue<T> {
            AsyncQueue {
                stuff: Rc::new(RefCell::new(AsyncQueueStuff {
                    queue: VecDeque::new(),
                    waker: None
                }))
            }
        }

        pub async fn pop(&mut self) -> T {
            // Construct a future to keep track of the pop request
            struct PopFuture<U> {
                stuff: Rc<RefCell<AsyncQueueStuff<U>>>
            }

            impl<U> Unpin for PopFuture<U> {}

            impl<U> Future for PopFuture<U> {
                type Output = U;

                fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
                    if self.as_ref().stuff.borrow().queue.is_empty() {
                        self.as_mut().stuff.borrow_mut().waker =
                            Some(cx.waker().clone());
                        Poll::Pending
                    } else {
                        Poll::Ready(self.as_mut().stuff.borrow_mut().queue.pop_front().unwrap())
                    }
                }
            }

            impl<U> Drop for PopFuture<U> {
                fn drop(&mut self) {
                    self.stuff.borrow_mut().waker = None;
                }
            }

            PopFuture { stuff: self.stuff.clone() }.await
        }

        pub fn push(&mut self, v: T) {
            let mut stuff = self.stuff.borrow_mut();
            stuff.queue.push_back(v);
            // Notify and reset waker
            if let Some(waker) = stuff.waker.take() {
                waker.wake();
            }
        }
    }

    static VTABLE: &RawWakerVTable = &RawWakerVTable::new(
        // clone
        | data: *const ()| RawWaker::new(data, VTABLE),
        // wake
        | _data: *const ()| {},
        // wake_by_ref
        | _data: *const ()| {},
        // drop
        | _data: *const ()| {}
    );

    fn create_waker() -> Waker {
        unsafe { Waker::from_raw(RawWaker::new(&(), VTABLE)) }
    }

    // Execute a future
    pub fn execute_one<F: Future>(mut fut: F) -> F::Output {
        let waker = create_waker();
        let mut ctx = Context::from_waker(&waker);
        let mut pf = unsafe { Pin::new_unchecked(&mut fut) };
        loop {
            if let Poll::Ready(v) = pf.as_mut().poll(&mut ctx) {
                return v;
            }
        }
    }

    enum MaybeFuture<F: Future> {
        Running(F),
        Completed(F::Output)
    }

    struct WaitMany<F: Future> {
        futs: Vec<MaybeFuture<F>>
    }

    impl<F: Future> Unpin for WaitMany<F> {}

    impl<F> Future for WaitMany<F> 
        where F: Future + Unpin
    {
        type Output = Vec<F::Output>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            let mut done = true;
            for fut in self.futs.iter_mut() {
                match fut {
                    MaybeFuture::Running(ref mut f) => {
                        match Future::poll(Pin::new(f), cx) {
                            Poll::Pending => {
                                done = false;
                                continue
                            },
                            Poll::Ready(v) => {
                                *fut = MaybeFuture::Completed(v);
                            }
                        }
                    },
                    MaybeFuture::Completed(_) => {}
                }
            }
            if done {
                Poll::Ready(
                    self.futs
                        .drain(..)
                        .map(|mf| {
                            match mf {
                                MaybeFuture::Completed(v) => v,
                                MaybeFuture::Running(_) => panic!("oops")
                            }
                        })
                        .collect()
                )
            } else {
                Poll::Pending
            }
        }
    }

    pub async fn wait_many<F: Future + Unpin>(futs: Vec<F>) -> Vec<F::Output> {
        WaitMany {
            futs: futs
                .into_iter()
                .map(|f| MaybeFuture::Running(f))
                .collect()
        }.await
    }

    //// Single global index of the next future to run
    //static mut FUT_IDX: usize = 0;
    //static INDEX_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
    //    // clone
    //    | data: *const ()| RawWaker::new(data, VTABLE),
    //    // wake
    //    | data: *const ()| unsafe { FUT_IDX = data as usize },
    //    // wake_by_ref
    //    | data: *const ()| unsafe { FUT_IDX = data as usize },
    //    // drop
    //    | _data: *const ()| {}
    //);

    //pub fn execute_many(futs: &[&mut dyn Future<Output=()>]) {
    //    loop {
    //        let idx = unsafe { FUT_IDX };
    //        let fut = &mut futs[idx];
    //        let waker = unsafe { Waker::from_raw(RawWaker::new(idx as *const(), INDEX_VTABLE)) };
    //        let mut pf = unsafe { Pin::new_unchecked(fut) };
    //        let mut ctx = Context::from_waker(&waker);
    //        if let Poll::Ready(_) = pf.as_mut().poll(&mut ctx) {
    //            // If this future is done, but there is no next future to
    //            // run, then all our futures must be complete.
    //            if unsafe { FUT_IDX } == idx {
    //                return;
    //            }
    //        }
    //    }
    //}
}

#[cfg(test)]
mod tests {
    use super::myasync::*;
    use std::pin::Pin;

    async fn print(msg: &str) {
        println!("I am printer: {}", msg);
    }

    async fn run(q: &mut AsyncQueue<u32>) {
        let v = q.pop().await;
        println!("I got thing from queue! {}", v);
    }

    async fn foo(q: &mut AsyncQueue<u32>) {
        println!("I ran an async function!");

        let p1 = Pin::from(Box::new(print("foo")));
        let p2 = Pin::from(Box::new(print("bar")));

        wait_many(vec![p1, p2]).await;

        run(q).await;

        q.pop().await;
        println!("I'll never make it. :(");
    }

    #[test]
    fn it_works() {
        let mut queue = AsyncQueue::new();
        queue.push(42u32);

        execute_one(foo(&mut queue));
    }
}
