use std::sync::mpsc::Sender;

pub struct TelnetClient {
    sender: Sender<TelnetData>,
}

impl TelnetClient {
    pub fn new(sender: Sender<TelnetData>) -> Self {
        Self {
            sender
        }
    }

    pub fn run(&mut self) {
        let mut client = telnet::Telnet::connect(("pkuxkx.net", 8081), 1024).expect("Couldn't connect to the server...");
        loop {
            let event = client.read_nonblocking().expect("Read error");
            if let telnet::Event::Data(buffer) = event {
                self.sender.send(TelnetData::new(buffer)).expect("send data error...");
            }
        }
    }
}

pub struct TelnetData {
    pub buf: Box<[u8]>,
}

impl TelnetData {
    pub fn new(buf: Box<[u8]>) -> Self {
        Self {
            buf
        }
    }
}