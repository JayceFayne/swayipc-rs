use super::common::receive_from_stream;
use crate::{Event, Fallible};
use async_io::Async;
use futures_lite::future::Boxed;
use futures_lite::ready;
use futures_lite::stream::Stream;
use std::os::unix::net::UnixStream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EventStream(Boxed<(Async<UnixStream>, Fallible<Event>)>);

async fn receive(mut stream: Async<UnixStream>) -> (Async<UnixStream>, Fallible<Event>) {
    let data = receive_from_stream(&mut stream).await;
    (stream, data.and_then(Event::decode))
}

impl EventStream {
    pub(super) fn new(stream: Async<UnixStream>) -> Self {
        Self(Box::pin(receive(stream)))
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (stream, item) = ready!(self.0.as_mut().poll(cx));
        self.0 = Box::pin(receive(stream));
        Poll::Ready(Some(item))
    }
}
