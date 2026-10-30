mod codecs;
mod client;
mod impl_client;
mod roaming;

use std::{env, sync::{Arc, atomic::AtomicU8}};
use crate::{client::Client, codecs::FragmentCodec, roaming::{EnvMapper, RoamingMapper, TryIntoFrom, TryMutAsyncFrom}};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use tokio_util::codec::FramedRead;

#[tokio::main]
async fn main() {
    let mapper = Box::leak(
            Box::new(
                EnvMapper::try_into_from(env::var("ROAMING").expect("Roaming env mapper is errored")).expect("Roaming env mapper is errored")
            )
        );

    let listener = TcpListener::bind(env::var("LISTENER").expect("Listener ENV is invalid!")).await.expect("Listener ENV is invalid!");

    loop {

        let (socket, _) = listener.accept().await.unwrap();
        socket.set_nodelay(true).unwrap();
        let (client_reader, client_writer) = socket.into_split();

        let mut reader_client_codec = FramedRead::new(client_reader, FragmentCodec {});

        let domain = RoamingMapper::try_mut_async_from(&mut reader_client_codec).await;

        match domain {
            Ok(mut vl) => {
                match mapper.get(&vl.domain) {
                    Some(out) => {
                        
                        let mut stream = TcpStream::connect(out).await.expect("Tcp connect to {out} failed");
                        stream.set_nodelay(true).unwrap();

                        stream.write_all_buf(&mut vl.bytes).await.unwrap();

                        let (server_reader, server_writer) = stream.into_split();


                        let state: Arc<AtomicU8> = Arc::new(AtomicU8::new(1));
                        let mut c2s_client = Client::new(reader_client_codec, server_writer, Arc::clone(&state));
                        let mut s2c_client = Client::new(FramedRead::new(server_reader, FragmentCodec {}), client_writer, Arc::clone(&state));

                        tokio::spawn(async move {
                            c2s_client.foo().await
                        });

                        tokio::spawn(async move {
                            s2c_client.foo().await
                        });

                    }
                    None => {
                        drop(reader_client_codec);
                        todo!("Logging")
                    }
                }
            }
            Err(e) => {
                drop(reader_client_codec);
                todo!("Logging")
            }
        }

    }
}