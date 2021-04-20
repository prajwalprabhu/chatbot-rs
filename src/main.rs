use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, channel};
use std::thread;
use std::thread::JoinHandle;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    member: String,
    chat: String,
}

struct ChatBot<'a> {
    stream:&'a Vec<TcpStream>,
    members:&'a Vec<String>,
    // threads: Vec<JoinHandle<Result<(), std::io::Error>>>,
    channel_rx: mpsc::Receiver<Message>,
    channel_tx: mpsc::Sender<Message>,
}

impl<'a> ChatBot<'a> {
    pub fn new() -> ChatBot<'a> {
        let stream: &Vec<TcpStream> = &Vec::new();
        let members: &Vec<String> = &Vec::new();
        // let threads: Vec<thread::JoinHandle<Result<(), std::io::Error>>> = Vec::new();
        // let channel:mpsc::Receiver<i32> =
        let (channel_tx, channel_rx) = channel();

        ChatBot {
            stream,
            members,
            channel_rx,
            channel_tx, // threads,
        }
    }
    pub fn start(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        // let chann = self.channel_rx.clone();
        thread::spawn(|| recv(self.channel_rx,&self.members,&self.stream));
        for stream in listener.incoming() {
            self.add(stream?)?;
        }
        Ok(())
    }
    fn add(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
        let message = serde_json::to_string(&Message {
            member: "root".to_string(),
            chat: "Welcome".to_string(),
        })?;
        let m = message.trim().as_bytes();
        stream.write(m)?;
        stream.flush()?;
        let mut buff = [0; 1024];
        stream.read(&mut buff)?;
        let mut buffer: Vec<u8> = Vec::new();
        for &i in buff.iter() {
            if i != 0 {
                buffer.push(i);
            } else {
                break;
            }
        }
        let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
        println!("{:?}", result);
        self.stream.push(stream.try_clone()?);
        self.members.push(result.member);
        let channel = self.channel_tx.clone();
        thread::spawn(move || listen(stream, channel));

        Ok(())
    }
    // fn recv(&self) -> std::io::Result<()> {
    //     loop {
    //         let result: Message = self.channel_rx.recv().unwrap();
    //         // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    //         for i in 0..self.members.len() {
    //             if self.members[i] == result.member {
    //                 continue;
    //             } else {
    //                 let message = serde_json::to_string(&result)?;
    //                 let mut str =self.stream[i].try_clone()?;
    //                 str.write(message.as_bytes())?;
    //                 str.flush()?;
    //             }
    //             break;
    //         }
    //     }
    //     // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    //     Ok(())
    // }
    // fn handle_client(&mut self)->std::io::Result<()> {
    //     println!("{:?}",stream);
    //     // let message = String::from("<html><h1>Hello World!</h1></html>");
    //     let message = format!("{:?}",Message{member:"root".to_string(),chat:"Welcome".to_string()});
    //     stream.write(message.as_bytes())?;
    //     stream.flush()?;
    //     Ok(())
    // }
}
fn recv(
    channel: mpsc::Receiver<Message>,
    members: &Vec<String>,
    stream: &Vec<TcpStream>,
) -> std::io::Result<()> {
    loop {
        let result: Message = channel.recv().unwrap();
        // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
        for i in 0..members.len() {
            if members[i] == result.member {
                continue;
            } else {
                let message = serde_json::to_string(&result)?;
                let mut str = stream[i].try_clone()?;
                str.write(message.as_bytes())?;
                str.flush()?;
            }
            break;
        }
    }
    // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    Ok(())
}
fn listen(mut stream: TcpStream, channel: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        let mut buff = [0; 1024];
        stream.read(&mut buff)?;
        let mut buffer: Vec<u8> = Vec::new();
        for &i in buff.iter() {
            if i != 0 {
                buffer.push(i);
            } else {
                break;
            }
        }
        let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
        println!("{:?}", result);
        channel.send(result.clone()).unwrap();
        if result.chat.to_uppercase() == "exit".to_string() {
            break;
        }

        // for i in 0..self.members.len() {
        //     if self.members[i] == result.member {
        //         continue;
        //     } else {
        //         let message = serde_json::to_string(&result)?;
        //         self.stream[i].write(message.as_bytes())?;
        //         self.stream[i].flush()?;
        //     }
        // break;
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    print!("Hell World!");
    let mut app = ChatBot::new();
    app.start()?;
    Ok(())
}
