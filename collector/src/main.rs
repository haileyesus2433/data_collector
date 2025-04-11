use std::{
    io::Write,
    sync::mpsc::{self, Sender},
    time::Instant,
};

use shared::{CollectorCommand, DATA_COLLECTOR_ADDRESS};

pub fn collect_data(tx: Sender<CollectorCommand>) {
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
            collector_id: 1234,
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

pub fn send(command: CollectorCommand) {
    let bytes = shared::encode(command);
    println!("Encoded {} bytes.", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();
    stream.write_all(&bytes).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel::<CollectorCommand>();

    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx);
    });

    while let Ok(command) = rx.recv() {
        send(command);
    }
}
