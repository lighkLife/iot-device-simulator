use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;
use log::{info, warn};

const LEAD: u8 = 0xfe;
const STOP: u8 = 0x16;
const START: u8 = 0x68;

pub struct Dlt64507Simulator {
    name: Arc<String>,
    port: Arc<u16>,
}

impl Dlt64507Simulator {
    pub fn new(name: String, port: u16) -> Dlt64507Simulator {
        Dlt64507Simulator {
            name: Arc::new(name),
            port: Arc::new(port),
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        let name = self.name.clone();
        let port = self.port.clone();
        thread::spawn(move || {
            start_server(name, port);
        })
    }

    pub fn stop() {}
}

fn start_server(name: Arc<String>, port: Arc<u16>) {
    let addr = "0.0.0.0:".to_string() + &port.to_string();
    let listener = TcpListener::bind(&addr).unwrap();
    info!("started listener DLT645-07 {}, {}", &name, &addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let name_clone = name.clone();
                thread::spawn(move || handle_connection(name_clone, stream));
            }
            Err(e) => {
                warn!("{} found error in stream, {}", &name, e);
            }
        }
    }
}

fn handle_connection(name: Arc<String>, stream: TcpStream) -> Result<()> {
    info!("connected from {}", stream.peer_addr()?);
    let stream_clone = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    loop {
        let mut request = vec![];
        reader.read_until(STOP, &mut request)?;

        if request.len() <= 0 {
            continue;
        }

        match request.iter().position(|&it| it == START) {
            None => info!("{} refuse to respond because of the request is invalid. receive= {:02X?}, \n", name, request),
            Some(index) => {
                let mut response = vec![];
                let address = &request[(index + 1)..(index + 7)];
                let data_id = &request[(index + 10)..(index + 14)];
                response.extend_from_slice(&[LEAD, LEAD]);
                response.extend_from_slice(&[START]);
                response.extend_from_slice(address);
                response.extend_from_slice(&[START]);
                response.extend_from_slice(&[0x91]);
                response.extend_from_slice(&[0x06]);
                response.extend_from_slice(&data_id);
                response.extend_from_slice(&[0x34, 0x36]);
                response.extend_from_slice(&[cs(&response)]);
                response.extend_from_slice(&[STOP]);
                info!("DLT645-07 {} \nreceive:  {:02X?}\nresponse: {:02X?}\n", name, request, response);
                writer.write(&response)?;
                writer.flush()?;
            }
        }
    }
}

fn cs(data: &Vec<u8>) -> u8 {
    let sum: u32 = data
        .iter()
        .filter(|&it| *it != LEAD)
        .map(|&it| it as u32)
        .sum();
    (sum % 256) as u8
}