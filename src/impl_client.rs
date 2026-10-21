use std::{pin::Pin, sync::{Arc, atomic::AtomicU8}, task::Poll};
use pin_project_lite::pin_project;
use futures_core::Stream;
use tokio::io::AsyncWrite;

use crate::codecs::PacketFrame;


pin_project! {
    pub struct ImplClient<T, D> 
    where
        T: Stream<Item = Result<PacketFrame, std::io::Error>>,
        D: AsyncWrite
    {
        #[pin]
        reader: T,
        #[pin]
        writer: D,
        state: Arc<AtomicU8>
    }
}

impl<T, D> ImplClient<T, D>
where 
    T: Stream<Item = Result<PacketFrame, std::io::Error>>,
    D: AsyncWrite
{

    pub fn new(reader: T, writer: D, state: Arc<AtomicU8>) -> ImplClient<T, D> {
        ImplClient { 
            reader: reader,
            writer: writer,
            state: state
        }
    }

    pub fn get_ref_reader(self: &mut Self) -> &T {
        &self.reader
    }

    pub fn get_mut_ref_reader(self: &mut Self) -> &mut T {
        &mut self.reader
    }

    pub fn get_ref_writer(self: &mut Self) -> &D {
        &self.writer
    }

    pub fn get_mut_ref_writer(self: &mut Self) -> &mut D {
        &mut self.writer
    }

    pub fn get_pinned_reader(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().reader
    }

    pub fn set_state(self: &mut Self, state: u8) {
        self.state.store(state, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_state(self: &mut Self) -> u8 {
        self.state.load(std::sync::atomic::Ordering::Relaxed)
    }

}

impl<T, D> Stream for ImplClient<T, D>
where 
    T: Stream<Item = Result<PacketFrame, std::io::Error>>,
    D: AsyncWrite
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        if self.state.load(std::sync::atomic::Ordering::Relaxed) == 0 {
            return Poll::Ready(None);
        }
        self.project().reader.poll_next(cx)
    }
}


impl<T, D> AsyncWrite for ImplClient<T, D> 
where 
    T: Stream<Item = Result<PacketFrame, std::io::Error>>,
    D: AsyncWrite
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.project().writer.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().writer.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().writer.poll_shutdown(cx)
    }
}