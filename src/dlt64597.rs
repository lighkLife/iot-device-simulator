use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;
use log::{debug, info, warn};

const LEAD: u8 = 0xfe;
const STOP: u8 = 0x16;
const START: u8 = 0x68;

pub struct Dlt64597Simulator {
    name: Arc<String>,
    port: Arc<u16>,
}

impl Dlt64597Simulator {
    pub fn new(name: String, port: u16) -> Dlt64597Simulator {
        Dlt64597Simulator {
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
        debug!("{} expect  : [FE, FE, FE, FE, 68, 57, 30, 60, 51, 00, 00, 68, 11, 02, 33, 34, ED, 16]", name);
        if request.len() > 0 {
            let mut response = vec![];
            let address = &request[5..11];
            let data_id = &request[14..18];
            response.extend_from_slice(&[LEAD, LEAD]);
            response.extend_from_slice(&[START]);
            response.extend_from_slice(address);
            response.extend_from_slice(&[START]);
            response.extend_from_slice(&[0x81]);
            response.extend_from_slice(&[0x06]);
            response.extend_from_slice(&data_id);
            response.extend_from_slice(&[0x34, 0x36]);
            response.extend_from_slice(&[cs(&response)]);
            response.extend_from_slice(&[STOP]);
            info!("{} response: {:02X?}\n", name, response);
            writer.write(&response)?;
            writer.flush()?;
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