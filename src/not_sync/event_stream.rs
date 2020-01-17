use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use async_std::os::unix::net::UnixStream;
use futures_core::future::BoxFuture;
use futures_core::ready;
use futures_core::stream::Stream;
use futures_util::FutureExt;
use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EventStream<'a> {
    stream: UnsafeCell<UnixStream>,
    state: Option<BoxFuture<'a, Fallible<(u32, Vec<u8>)>>>,
}

impl EventStream<'_> {
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream: UnsafeCell::new(stream),
            state: None,
        }
    }

    fn queue_next_event(&mut self) {
        self.state = Some(receive_from_stream(unsafe { &mut *self.stream.get() }).boxed())
    }
}

impl Stream for EventStream<'_> {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            Some(fut) => {
                let res = ready!(fut.as_mut().poll(cx));
                self.queue_next_event();
                Poll::Ready(Some(match res {
                    Err(err) => Err(err),
                    Ok(raw_event) => Event::try_from(raw_event),
                }))
            }
            None => {
                self.queue_next_event();
                Poll::Pending
            }
        }
    }
}
