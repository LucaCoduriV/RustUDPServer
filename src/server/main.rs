use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use byteorder::{ByteOrder, NetworkEndian};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("[::]:3000")?;  // for UDP4/6

    let mut buf = [0; 2048];
    let clients : Vec<SocketAddr> = vec![];

    let (tx, rx) = mpsc::channel();

    thread::spawn(|| thread_function(rx));

    loop {

        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        socket.send_to(buf, &src)?;
    }
    let mut i = 0;
    loop {
        tx.send(i).expect("TODO: panic message");
        i += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread_function(rx: mpsc::Receiver<i32>){
    for received in rx {
        println!("{}", received)
    }
}