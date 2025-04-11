use std::{
    collections::VecDeque,
    io::Write,
    sync::mpsc::{self, Sender},
    time::Instant,
};

use shared::{CollectorCommand, DATA_COLLECTOR_ADDRESS, encode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("unable to connect to the server")]
    UnableToConnect,
    #[error("unable to send to the server")]
    UnableToSend,
}

pub fn collect_data(tx: Sender<CollectorCommand>, collector_id: u128) {
    let mut sys = sysinfo::System::new_all();

    sys.refresh_memory();
    sys.refresh_cpu_all();

    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    loop {
        let now = Instant::now();

        sys.refresh_memory();
        sys.refresh_cpu_all();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        let send_result = tx.send(CollectorCommand::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error Sending Data: {e:?}");
        }

        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}

pub fn send(command: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(&command)
        .map_err(|_| CollectorError::UnableToSend)?;
    Ok(())
}

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
    }
    Ok(())
}

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

fn main() {
    let uuid = get_uuid();
    let (tx, rx) = mpsc::channel::<CollectorCommand>();

    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx, uuid);
    });
    let mut data_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = encode(&command);
        data_queue.push_back(encoded);
        let _ = send_queue(&mut data_queue);
        // while let Some(encoded) = data_queue.pop_front() {
        //     if send(&encoded).is_err() {
        //         println!("Error sending command");
        //         data_queue.push_front(encoded);
        //         break;
        //     }
        // }
    }
}
