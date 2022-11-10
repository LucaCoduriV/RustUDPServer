use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("[::]:3000")?;  // for UDP4/6
    let mut buf = [0; 2048];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        socket.send_to(buf, &src)?;
    }
}