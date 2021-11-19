use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};

use yi_dns::Message;
fn main() {
    let mut socket = UdpSocket::bind("10.20.11.37:30000").unwrap();


    // socket.send_to(&[ 0x00,0x00,0x00, 0x01,0xc0,0xa8,0x2,0x1,0x00, 0x35,
    //     0xff ,0xff, 01, 00 ,00 ,01, 00, 00, 00, 00, 00, 00, 03, 0x70 ,0x61, 0x6e,
    //     05, 0x62, 0x61, 0x69, 0x64, 0x75, 03, 0x63, 0x6f ,0x6d, 00, 00, 01, 00, 01 ], "127.0.0.1:1030");


        
    // socket.send_to(&[ 0x00,0x00,0x00, 0x01,0x8,0x8,0x8,0x8,0x00, 0x35,
    //     0xff ,0xff, 01, 00 ,00 ,01, 00, 00, 00, 00, 00, 00, 03, 0x70 ,0x61, 0x6e,
    //     05, 0x62, 0x61, 0x69, 0x64, 0x75, 03, 0x63, 0x6f ,0x6d, 00, 00, 01, 00, 01 ], "127.0.0.1:3000");
    // socket.send_to(&[ 0xff ,0xff, 01, 00 ,00 ,01, 00, 00, 00, 00, 00, 00, 03, 0x70 ,0x61, 0x6e,
    //     05, 0x62, 0x61, 0x69, 0x64, 0x75, 03, 0x63, 0x6f ,0x6d, 00, 00, 01, 00, 01], "10.151.6.254:53");

    let query = Message::new_query_ns("www.sohu.com");
    socket.send_to(&query.to_bytes().to_vec(), "10.151.6.254:53");

    let mut buf = [0u8;1024];
    
    match socket.recv(&mut buf) {
        Ok(received) =>{
            println!("received {} bytes {:?}", received, &buf[..received]);
            let message =  Message::from_bytes(buf[..received].to_vec()).unwrap();
            println!("{:#?}", message);
        } 
        
        Err(e) => println!("recv function failed: {:?}", e),
    }

      
    // 0010   
    
    println!("Hello, world!");

  

}
