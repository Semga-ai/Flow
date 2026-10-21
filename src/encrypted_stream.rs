use std::{pin::Pin, task::Poll};

use futures::AsyncWrite;
use tokio::io::AsyncRead;
use aes::Aes128;
use cfb8::cipher::{KeyIvInit};

type Aes128Cfb8Enc = cfb8::Encryptor<Aes128>;
type Aes128Cfb8Dec = cfb8::Decryptor<Aes128>;

pub struct EncryptedStream<T> {
    inner: T,
    decoder: Option<Aes128Cfb8Dec>,
    encoder: Option<Aes128Cfb8Enc>
}


impl <T> EncryptedStream<T> {
    
    /// Creates a new [`EncryptedStream<T>`].
    pub fn new(inner: T) -> EncryptedStream<T> {
        EncryptedStream {
            inner: inner,
            decoder: None,
            encoder: None
        }
    }

    pub fn enable_crypte(self: &mut Self, key: [u8; 16]) {
        self.decoder = Some(Aes128Cfb8Dec::new_from_slices(&key, &key).unwrap());
        self.encoder = Some(Aes128Cfb8Enc::new_from_slices(&key, &key).unwrap())
    }

}


impl <T: AsyncRead + Unpin> AsyncRead for EncryptedStream<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let foo = buf.filled().len();

        let selff = self.get_mut();

        let decoder: &mut Option<cfb8::Decryptor<Aes128>> = &mut selff.decoder;

        let poll = Pin::new(&mut selff.inner).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = poll {
            if let Some(decoder) = decoder {
                let bar = buf.filled_mut();

                let baz = &mut bar[foo..];
                decoder.decrypt(baz);
            }
        }

        poll
    }
}


impl <T: AsyncWrite + Unpin> AsyncWrite for EncryptedStream<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let selff = self.get_mut();
        let foo = Pin::new(&mut selff.inner);
        if let Some(encoder) = &mut selff.encoder {
            let bar = &mut buf.to_vec();
            encoder.encrypt(bar);
            foo.poll_write(cx, bar)
        } else {
            foo.poll_write(cx, buf)
        }
        
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_close(cx)
    }
}