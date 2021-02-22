#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use std::net::TcpStream;
pub fn is_open(host : &str, port:u16) -> bool{
    let adr = format!("{}:{}",host,port);
    return TcpStream::connect(adr).is_ok();
}
