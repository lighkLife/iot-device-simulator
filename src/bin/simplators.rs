use std::env;
use log::LevelFilter;

use iot_device_simulator::{Dlt64507Simulator, Dlt64597Simulator};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    let count = match args.len() {
        2 => *(&args[1].trim().parse::<u16>().expect("expect a u16")),
        _ => 1
    };

    let mut handles = Vec::new();
    for i in 0..count {
        let dlt64507_simulator = Dlt64507Simulator::new(String::from((18500 + i).to_string()), 18500 + i);
        handles.push(dlt64507_simulator.start());

        let dlt64597_simulator = Dlt64597Simulator::new(String::from((19500 + i).to_string()), 19500 + i);
        handles.push(dlt64597_simulator.start());

    }
    for handle in handles {
        let _ = handle.join();
    }
}


