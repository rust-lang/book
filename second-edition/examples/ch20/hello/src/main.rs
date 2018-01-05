use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    //let mut i: u32 = 0;
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
      //  i = i + 1;
      //  println!("Connection {} established!", i);
		println!("Connection  established!");
    }
}