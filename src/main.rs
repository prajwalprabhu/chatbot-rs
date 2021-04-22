#![feature(mutex_unlock)]
use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, channel};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::thread::JoinHandle;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    member: String,
    chat: String,
}

struct ChatBot {
    stream: Arc<Mutex<Vec<TcpStream>>>,
    members: Arc<Mutex<Vec<String>>>,
    // threads: Vec<JoinHandle<Result<(), std::io::Error>>>,
    // channel_rx: mpsc::Receiver<Message>,
    // channel_tx: mpsc::Sender<Message>,
}

impl ChatBot {
    pub fn new() -> ChatBot {
        let stream: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
        let members: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        // let threads: Vec<thread::JoinHandle<Result<(), std::io::Error>>> = Vec::new();
        // let channel:mpsc::Receiver<i32> =

        ChatBot {
            stream,
            members,
            // channel_rx,
            // channel_tx, // threads,
        }
    }
    pub fn start(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let (channel_tx, channel_rx) = channel();

        // let chann = self.channel_rx.clone();
        let member = Arc::clone(&self.members);
        let stream = Arc::clone(&self.stream);
        let start_thread =
            thread::spawn(|| recv(channel_rx, member, stream).expect("start thread said"));

        for stream in listener.incoming() {
            self.add(stream?, channel_tx.clone())?;
        }
        start_thread.join().expect("start thread join");
        Ok(())
    }
    fn add(
        &mut self,
        mut stream: TcpStream,
        channel_tx: mpsc::Sender<Message>,
    ) -> std::io::Result<()> {
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
        let mut stream_vec = self.stream.lock().unwrap();
        stream_vec.push(stream.try_clone()?);
        // self.stream.
        println!("stream {:?}", stream_vec);
        Mutex::unlock(stream_vec);
        let mut members_vec = self.members.lock().unwrap();
        members_vec.push(result.member);
        println!("member {:?}", members_vec);
        Mutex::unlock(members_vec);
        let channel = channel_tx.clone();
        thread::spawn(move || listen(stream, channel).expect("listen failed "));

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
    members: Arc<Mutex<Vec<String>>>,
    stream: Arc<Mutex<Vec<TcpStream>>>,
) -> std::io::Result<()> {
    loop {
        let result: Message = channel.recv().unwrap();
        let mut members = members.lock().unwrap();
        let mut stream = stream.lock().unwrap();
        println!("Result recv channel{:?}", result);
        // println!("member {:?}",members);
        // println!("stream {:?}",stream);
        // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();

        for i in 0..members.len() {
            // println!("Member {:?} ",members[i]);
            println!("for");
            let message = serde_json::to_string(&result).expect("message ");
            println!("Message : {:?} to {:?}", message, members[i]);
            // let mut str = stream[i].try_clone()?;
            if stream[i].write(message.as_bytes()).is_err(){
                stream.remove(i);
                members.remove(i);
            }
            stream[i].flush()?;
            // }
            // break;
        }
        Mutex::unlock(members);
        Mutex::unlock(stream);
    }

    // let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    Ok(())
}
fn listen(mut stream: TcpStream, channel: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        let mut buff = [0; 1024];
        stream.read(&mut buff)?;
        if buff[0] == 0 {
            continue;
        }
        let mut buffer: Vec<u8> = Vec::new();

        for &i in buff.iter() {
            if i != 0 {
                buffer.push(i);
            } else {
                break;
            }
        }
        let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
        // println!("{:?}", result);
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
