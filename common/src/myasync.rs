use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::future::*;
use std::pin::Pin;
use std::task::*;
//use magnetic::spsc::*;
//use magnetic::buffer::*;

struct ChannelStuff<T> {
    queue: VecDeque<T>,
    // Tasks waiting on data from this queue
    waker: Option<Waker>
}

pub struct Sender<T> {
    stuff: Rc<RefCell<ChannelStuff<T>>>
}

impl<T> Sender<T> {
    pub fn send(&mut self, v: T) {
        let mut stuff = self.stuff.borrow_mut();
        stuff.queue.push_back(v);
        // Notify and reset waker
        if let Some(waker) = stuff.waker.take() {
            waker.wake();
        }
    }
}

pub struct Receiver<T> {
    stuff: Rc<RefCell<ChannelStuff<T>>>
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> T {
        // Construct a future to keep track of the pop request
        struct PopFuture<U> {
            stuff: Rc<RefCell<ChannelStuff<U>>>
        }

        impl<U> Unpin for PopFuture<U> {}

        impl<U> Future for PopFuture<U> {
            type Output = U;

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
                let elem = self.stuff.borrow_mut().queue.pop_front();
                match elem {
                    Some(v) => Poll::Ready(v),
                    None => {
                        self.stuff.borrow_mut().waker = Some(cx.waker().clone());
                        Poll::Pending
                    }
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
}

pub fn make_channel<T>() -> (Sender<T>, Receiver<T>) {
    let stuff = Rc::new(RefCell::new(ChannelStuff {
        queue: VecDeque::new(),
        waker: None
    }));
    (Sender{stuff: stuff.clone()}, Receiver{stuff: stuff.clone()})
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

enum MaybeComplete<T> {
    Running(Pin<Box<dyn Future<Output=T>>>),
    Completed(T),
}

impl<T> Unpin for MaybeComplete<T> {}

impl<T> MaybeComplete<T> {
    fn take(self) -> Option<T> {
        match self {
            MaybeComplete::Running(_) => None,
            MaybeComplete::Completed(v) => Some(v)
        }
    }
}

impl<T> Future for MaybeComplete<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let me = unsafe { self.get_unchecked_mut() };
        if let MaybeComplete::Running(f) = me {
            if let Poll::Ready(v) = f.as_mut().poll(cx) {
                *me = MaybeComplete::Completed(v);
                return Poll::Ready(())
            }
        }
        Poll::Pending
    }
}

struct WaitMany<T> {
    futs: Vec<MaybeComplete<T>>
}

impl<T> Future for WaitMany<T> {
    type Output = Vec<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut done = true;
        for fut in self.futs.iter_mut() {
            let fp = Pin::new(fut);
            match fp.poll(cx) {
                Poll::Pending => {
                    done = false;
                    continue
                }
                Poll::Ready(_) => {}
            }
        }

        if done {
            Poll::Ready(
                self.futs
                    .drain(..)
                    .map(|mf| mf.take().unwrap())
                    .collect()
            )
        } else {
            Poll::Pending
        }
    }
}

pub async fn wait_many<T>(futs: Vec<Pin<Box<dyn Future<Output=T>>>>) -> Vec<T> {
    WaitMany {
        futs: futs
            .into_iter()
            .map(|f| MaybeComplete::Running(f))
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