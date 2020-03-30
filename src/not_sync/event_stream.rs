use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use async_std::os::unix::net::UnixStream;
use futures_core::future::BoxFuture;
use futures_core::ready;
use futures_core::stream::Stream;
use futures_util::FutureExt;
use std::convert::TryFrom;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EventStream(BoxFuture<'static, (UnixStream, Fallible<(u32, Vec<u8>)>)>);

async fn receive(mut stream: UnixStream) -> (UnixStream, Fallible<(u32, Vec<u8>)>) {
    let data = receive_from_stream(&mut stream).await;
    (stream, data)
}

impl EventStream {
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self(receive(stream).boxed())
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (stream, res) = ready!(self.0.as_mut().poll(cx));
        self.0 = receive(stream).boxed();
        Poll::Ready(Some(res.and_then(Event::try_from)))
    }
}
