use std::net::SocketAddr;

use shared::{DATA_COLLECTOR_ADDRESS, decode};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub async fn data_collector() -> anyhow::Result<()> {
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;
    loop {
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
    println!("new connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("Failed to read data from socket");
        if n == 0 {
            println!("No Data received - connection closed");
            return;
        }
        println!("Received {n} bytes");
        let received_data = decode(&buf[0..n]);
        println!("received data: {received_data:?}")
    }
}
