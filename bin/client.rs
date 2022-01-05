use prost::Message;
use prost::encoding::sint32::encode;
use rand::Rng;
use tokio::net::UdpSocket;
use tokio::io;
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use rand;

use tokio_stream_test::protogen::echo::EchoMsg;


#[tokio::main]
async fn main() -> io::Result<()> { 
    let mut rng = rand::thread_rng();
    let sock;
    loop{
        let port = rng.gen_range(10000..65536);

        match UdpSocket::bind(&format!("0.0.0.0:{}", &port)).await{
            Ok(s) => {
                sock = s;
                break;
            },
            Err(e)=> println!("port {} is in use, try again: {}", port, e),
        }
    };


    let sock = Arc::new(sock);

    let addr = "0.0.0.0:8080";
    sock.connect(addr).await?;

    let mut input = io::stdin();
    let sc = sock.clone();
    tokio::spawn(async move{
        let mut buf = vec![0;1000];
        loop{
            let len = sc.recv(&mut buf).await.unwrap();
            let msg = EchoMsg::decode(&buf[..len]);

            println!("received {} bytes: `{:?}`", len, msg);
        }
    });

    loop{
        let mut s = vec![];
        input.read_buf(&mut s).await.unwrap();
        println!("received input from stdin: {:?}", std::str::from_utf8(&s));

        let mut msg = EchoMsg::default();
        msg.chat = std::str::from_utf8(&s).unwrap().to_owned();

        let mut buf = vec![];
        msg.encode(&mut buf).unwrap();

        println!("encoded buffer size `{:?}`", &buf);
        
        sock.send(&mut buf).await?; 
    }

    Ok(())
}