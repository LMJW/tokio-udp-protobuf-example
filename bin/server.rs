use prost::Message;
use tokio::net::UdpSocket;
use std::{io};
use tokio_stream_test::protogen::echo::EchoMsg;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    let mut clients = HashSet::new();

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        if !clients.contains(&addr){
            clients.insert(addr.clone());
        }
        println!("{:?} bytes received from {:?}", len, addr);
        println!("{:?} --- {}", &buf[..len], len);
        let mut msg = EchoMsg::decode(&buf[..len]).unwrap();
        msg.ipv4 = addr.to_string();

        let len = msg.encoded_len();
        msg.encode(&mut buf.as_mut()).unwrap();
        for addr in &clients{
            let len = sock.send_to(&buf[..len], addr).await?;
            println!("{:?} bytes sent to {:?}", len, addr);
        }
    }
}