
use {
    std::{
        io,
        net::{
            TcpStream, TcpListener,
        },
    },
};


pub struct Socket {
    listener: std::net::TcpListener,
}

impl Socket {

    pub fn new(&self) -> io::Result<Socket> {
        //connect to Tcp rsync socket :873
        let listener = TcpListener::bind("127.0.0.1:873")?;
        Ok(
            Socket {
                listener,
            }
        )
    }

    pub fn connect(){
        //return TcpStream
    }

    
}
