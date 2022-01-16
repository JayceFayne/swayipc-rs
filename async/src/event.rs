use super::common::receive_from_stream;
use crate::runtime::Socket;
use crate::{Event, Fallible};
use futures_lite::future::Boxed;
use futures_lite::ready;
use futures_lite::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EventStream(Boxed<(Socket, Fallible<Event>)>);

async fn receive(mut stream: Socket) -> (Socket, Fallible<Event>) {
    let data = receive_from_stream(&mut stream).await;
    (stream, data.and_then(Event::decode))
}

impl EventStream {
    pub(super) fn new(stream: Socket) -> Self {
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
