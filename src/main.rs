extern crate futures;

use futures::executor::ThreadPool;
use futures::prelude::*;
use futures::task::Context;
use futures::future::FutureResult;
use futures::future;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Frame(i32);

impl IntoFuture for Frame {
    type Item = Frame;
    type Error = FrameError;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        // general frame stuff here
        future::ok(self)
    }
}

#[derive(Debug)]
struct FrameError;

impl Error for FrameError {
    fn description(&self) -> &str {
        "An error occured in this frame"
    }
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameError")
    }
}

struct Engine;

impl Engine {
    fn new() -> Self {
        Engine
    }
    fn frames(&self) -> FrameStream {
        FrameStream { count: 0i32 }
    }
}

#[derive(Debug)]
struct FrameStream {
    count: i32,
}

impl Stream for FrameStream {
    type Item = Frame;
    type Error = FrameError;
    fn poll_next(&mut self, _cx: &mut Context) -> Poll<Option<Self::Item>, Self::Error> {
        self.count += 1;
        Ok(Async::Ready(Some(Frame(self.count))))
    }
}

fn main() {
    let mut pool = ThreadPool::new().expect("Failed to create threadpool");

    let engine = Engine::new()
        .frames()
        .map(|frame| {
            println!("Frame:{:?}", frame);
            frame
        })
        .map_err(|e| println!("failed to create frame; error = {:?}", e))
        .take(3)
        .collect::<Vec<Frame>>();


    let res = pool.run(engine);
    println!("res: {:?}", res);
}
