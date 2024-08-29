use std::net::UdpSocket;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "config.json")]
    config: String,

    #[clap(short, long, default_value = "false")]
    enable_debug: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    tokens: Vec<String>,
    target: String,
    port: u32,
    bind: String,
    // TODO: multithreading
}
fn main() {
    let args = Args::parse();
    // check if file exists
    let config_path = std::fs::read_to_string(args.config).expect("File not found");
    let config: Config = serde_json::from_str(&config_path).expect("Invalid JSON");

    // a udp proxy
    let socket = UdpSocket::bind(format!("{}:{}", config.bind, config.port))
        .expect("Could not bind to address");
    println!("Listening on {}", socket.local_addr().unwrap());
    let mut buf = [0; 1024];

    // recv packets and return them back to the client
    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        let buf = &mut buf[..amt];
        if args.enable_debug {
            println!("Received {} bytes from {}", amt, src);
        }

        // <token>::<data>
        let message = std::str::from_utf8(&buf).expect("Invalid UTF-8");
        let parts: Vec<&str> = message.split("::").collect();
        if parts.len() < 2 {
            if args.enable_debug {
                println!("Missing authentication token ({message})");
            }
            continue;
        }

        let token = parts[0];
        if !config.tokens.contains(&token.to_string()) {
            if args.enable_debug {
                println!("Invalid token ({token})");
            }
            continue;
        }

        // the rest of the data after the token
        let data = parts[1..].join("::");
        if args.enable_debug {
            println!("Token: {}, Data: {}", token, data);
        }

        let rest = data.as_bytes();
        // forward the data to the target
        let sent = socket
            .send_to(rest, config.target.as_str())
            .expect("Failed to send data");
        if args.enable_debug {
            println!("Sent {} bytes to {}", sent, config.target);
        }
    }
}
