use std::env;
use log::LevelFilter;

use iot_device_simulator::Dlt64507Simulator;

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
        let dlt64507_simulator = Dlt64507Simulator::new(String::from((9500 + i).to_string()), 9500 + i);
        let join_handle = dlt64507_simulator.start();
        handles.push(join_handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}


