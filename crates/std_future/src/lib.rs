// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

// A simple future that resolves after a specified duration
struct TimerFuture {
    expiration_time: Instant,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        Self {
            expiration_time: Instant::now() + duration,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Timer future completed!");
            Poll::Ready(())
        } else {
            // In a real executor, we would register a waker here
            println!("Timer future not ready yet, polling again...");

            // For simplicity, we'll just sleep briefly and then manually re-poll
            thread::sleep(Duration::from_millis(100));
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// A simple async function that returns a future
async fn async_task() {
    println!("Starting async task");

    // Use the TimerFuture to wait for 2 seconds
    TimerFuture::new(Duration::from_secs(2)).await;

    println!("Async task completed after waiting");
}

// Simple executor to run our future
fn block_on<F: Future>(mut future: F) -> F::Output {
    use std::sync::Arc;
    use std::task::Wake;

    // Create a waker that just calls wake_by_ref on itself
    struct DummyWaker;
    impl Wake for DummyWaker {
        fn wake(self: Arc<Self>) {
            // Do nothing
        }
    }

    let waker = Arc::new(DummyWaker).into();
    let mut context = Context::from_waker(&waker);

    // Pin the future and poll it repeatedly until it's ready
    let mut pinned = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // In a real executor, we would yield the thread here
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_future_main() {
    println!("=== Rust std::Future example ===");

    println!("Running async task using our simple executor...");
    block_on(async_task());

    println!("Execution complete!");
}
