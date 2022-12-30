use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;
use log::{debug, info, warn};

const STOP: u8 = 0x16;

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
    info!("started listener {}, {}", &name, &addr);
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
        info!("{} receive : {:02X?}", name, request);
        debug!("{} expect  : [FE, FE, FE, FE, 68, 57, 30, 60, 51, 00, 00, 68, 11, 04, 33, 36, 34, 35, EF, 16]", name);

        let mut response = vec![];
        let address = &request[5..11];
        response.extend_from_slice(&[0xfe, 0xfe, 0xfe, 0xfe]);
        response.extend_from_slice(&[0x68]);
        response.extend_from_slice(address);
        response.extend_from_slice(&[0x68]);
        response.extend_from_slice(&[0x91]);
        response.extend_from_slice(&[0x06, 0x33, 0x36, 0x34, 0x35, 0xC5, 0x54]);
        response.extend_from_slice(&[cs(&response)]);
        response.extend_from_slice(&[0x16]);
        info!("{} response: {:02X?}\n", name, response);
        for _ in 0..100 {
            writer.write(&response)?;
        }
        writer.flush()?;
    }
}

fn cs(data: &Vec<u8>) -> u8 {
    let sum: u32 = data
        .iter()
        .filter(|&it| *it != 0xfe)
        .map(|&it| it as u32)
        .sum();
    (sum % 256) as u8
}