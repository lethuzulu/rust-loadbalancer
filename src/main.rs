use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
struct ServerPool {
    servers: Vec<Server>,
    sender: Sender<TcpStream>,
}

impl ServerPool {
    fn new(capacity: usize) -> ServerPool {
        //TODO: validate capacity > 0s
        let mut servers = Vec::with_capacity(capacity);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..capacity {
            let server = Server::new(id, Arc::clone(&receiver));
            servers.push(server);
        }
        ServerPool { servers, sender }
    }

    fn execute(&self, stream: TcpStream) {
        let _ = self.sender.send(stream).expect("Data to be sent");
    }
}

#[derive(Debug)]
struct Server {
    id: usize,
    thread: JoinHandle<()>,
}

impl Server {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<TcpStream>>>) -> Server {
        let listener = move || loop {
            let stream = receiver
                .lock()
                .expect("Lock to be released")
                .recv()
                .expect("Message");
            let port = 5000 + id;
            let address = format!("127.0.0.1:{}", port);
            let listener = TcpListener::bind(&address).unwrap();

            
        };
        let thread = thread::spawn(listener);
        Server { id, thread }
    }
}

#[derive(Debug)]
struct Job {
    stream: TcpStream,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    let pool = ServerPool::new(2);
    println!("pool {:?}", pool);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    println!("{:?}", stream);
}
