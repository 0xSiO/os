use core::{
    pin::Pin,
    task::{Context, Poll},
};

use conquer_once::spin::Lazy;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker};
use log::warn;

const SCANCODE_QUEUE_SIZE: usize = 100;

// The queue must be accessible only through immutable borrows, so it is not stored in
// ScancodeQueue, which requires a mutable borrow in order to call next().
static SCANCODE_QUEUE: Lazy<ArrayQueue<u8>> = Lazy::new(|| ArrayQueue::new(SCANCODE_QUEUE_SIZE));
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) struct ScancodeQueue;

impl ScancodeQueue {
    /// Called by the keyboard interrupt handler. Must not block or allocate.
    pub(crate) fn add_scancode(scancode: u8) {
        if Lazy::is_initialized(&SCANCODE_QUEUE) {
            SCANCODE_QUEUE
                .push(scancode)
                .unwrap_or_else(|_| warn!("Scancode queue is full; dropping keyboard input"));
            WAKER.wake();
        } else {
            warn!(
                "Couldn't add scancode {:#x}: queue is uninitialized!",
                scancode
            );
        }
    }
}

impl Stream for ScancodeQueue {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<u8>> {
        if let Some(scancode) = SCANCODE_QUEUE.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(context.waker());

        match SCANCODE_QUEUE.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}
