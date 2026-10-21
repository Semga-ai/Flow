use std::{sync::{Arc, atomic::AtomicU8}};

use futures::StreamExt;
use futures_core::Stream;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{codecs::PacketFrame, impl_client::ImplClient};

    pub struct Client<T: Stream<Item = Result<PacketFrame, std::io::Error>>, D: AsyncWrite> 
    {
        inner: ImplClient<T, D>
    }
impl <T, D> Client<T, D> 
where
    T: Stream<Item = Result<PacketFrame,std::io::Error>> + Unpin,
    D: AsyncWrite + Unpin
{
    /// Creates a new [`Client<T, D>`].
    pub fn new(reader: T, writer: D, state: Arc<AtomicU8>) -> Client<T, D> {
        Client {
            inner: ImplClient::new(reader, writer, state)
        }
    }

    /// Returns a mutable reference to the get mut ref inner of this [`Client<T, D>`].
    pub fn get_mut_ref_inner(self: &mut Self) -> &mut ImplClient<T, D> {
        &mut self.inner
    }

    pub async fn foo(self: &mut Self) {
        while let Some(Ok(mut frame)) = self.inner.get_mut_ref_reader().next().await {
            match self.inner.get_state() {
                0 => {
                    //terminate
                    todo!()
                },
                1 => {
                    //handshake
                },
                2 => {
                    //status
                },
                3 => {
                    //login 
                },
                _ => {
                    //other
                }
            }

            self.inner.get_mut_ref_writer().write_all_buf(&mut frame.bytes).await.unwrap();
        }
        self.inner.set_state(0);
    }
}