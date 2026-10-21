mod codecs;
mod network;
mod client;
mod encrypted_stream;
mod impl_client;

use std::{env, sync::{Arc, atomic::AtomicU8}};
use crate::{client::Client, codecs::{FragmentCodec}};
use tokio::{net::{TcpListener, TcpStream}};
use tokio_util::codec::FramedRead;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(env::var("LISTENER_ADDRESS").expect("Listener address is invalid!")).await.expect("Listener address is invalid!");

    loop {

        let (socket, _) = listener.accept().await.unwrap();
        socket.set_nodelay(true).unwrap();
        let (client_reader, client_writer) = socket.into_split();

        let stream = TcpStream::connect(env::var("CONNECTION_ADDRESS").expect("Connection_address is invalid!")).await.expect("Connection_address is invalid!");
        stream.set_nodelay(true).unwrap();

        let (server_reader, server_writer) = stream.into_split();


        let state: Arc<AtomicU8> = Arc::new(AtomicU8::new(1));
        let mut c2s_client = Client::new(FramedRead::new(client_reader, FragmentCodec {}), server_writer, Arc::clone(&state));
        let mut s2c_client = Client::new(FramedRead::new(server_reader, FragmentCodec {}), client_writer, Arc::clone(&state));

        tokio::spawn(async move {
            c2s_client.foo().await
        });

        tokio::spawn(async move {
            s2c_client.foo().await
        });

    }
}
