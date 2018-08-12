use futures::{task, Async, Future, Poll};
use std::sync::{Arc, Mutex};
use ServerStatus;

pub struct SignalFuture(pub Arc<Mutex<ServerStatus>>);

impl Future for SignalFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut status = self.0.lock().unwrap();

        if status.got_request {
            Ok(Async::Ready(()))
        } else {
            status.task = Some(task::current());
            Ok(Async::NotReady)
        }
    }
}
