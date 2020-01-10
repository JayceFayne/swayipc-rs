use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use async_std::os::unix::net::UnixStream;
use futures_core::ready;
use futures_core::stream::Stream;
use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub async fn next_event(stream: &mut UnixStream) -> Fallible<Event> {
    Event::try_from(receive_from_stream(stream).await?)
}

pub struct EventStream {
    stream: UnsafeCell<UnixStream>,
    state: Option<Pin<Box<dyn Future<Output = Fallible<Event>>>>>,
}

impl EventStream {
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream: UnsafeCell::new(stream),
            state: None,
        }
    }

    fn queue_next_event(&mut self) {
        self.state = Some(Box::pin(next_event(unsafe { &mut *self.stream.get() })))
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            Some(fut) => {
                let res = ready!(fut.as_mut().poll(cx));
                self.queue_next_event();
                Poll::Ready(Some(res))
            }
            None => {
                self.queue_next_event();
                Poll::Pending
            }
        }
    }
}
