use std::net::TcpStream;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;

fn main() {
    if let Ok(irc_stream) = TcpStream::connect("irc.root-me.org:6667") {
        let stream_rx_ptr = &irc_stream;
        let mut stream_tx_ptr = &irc_stream;
        let mut reader = BufReader::new(stream_rx_ptr);
        println!("Connected to the server!\r\n");

        stream_tx_ptr.write_all(b"NICK guest1436\r\n").unwrap();
        stream_tx_ptr.write_all(b"USER guest1436 * * :Guest\r\n").unwrap();

        crossbeam::scope(|scope| {
            let (s, r) = bounded(0);

            let rx_thread = scope.spawn(move |_| {
                let s = s.clone();
                let mut line = String::new();

                loop {
                    line.clear();
                    reader.read_line(&mut line).unwrap();
                    match line.split_whitespace().skip(1).next(){
                        Some("376") => s.send(Box::new("E1".to_owned())).unwrap(),
                        Some("PRIVMSG") => s.send(Box::new(line.clone())).unwrap(),
                        _ => {}
                    }
                }
            });

            let tx_thread = scope.spawn(move |_| {
                let r = r.clone();
                loop {
                    match r.recv() {
                        Ok(boxed_msg) => {
                            if *boxed_msg == "E1" {
                                stream_tx_ptr.write_all(b"JOIN #root-me_challenge\r\n").unwrap();
                                stream_tx_ptr.write_all(b"PRIVMSG candy !ep1\r\n").unwrap();
                            }
                            else {println!("{}", *boxed_msg);}
                        },
                        Err(_) => println!("Error in thread message passing"),
                    }
                }
            });        
            //stream_tx_ptr.write_all(b"QUIT\r\n").unwrap();

            rx_thread.join().unwrap();
            tx_thread.join().unwrap();
        })
        .unwrap();
        
        

    } else {
        println!("Couldn't connect to server...");
    }
}