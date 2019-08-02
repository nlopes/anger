use std::{thread, time};

use rand;
use rand::seq::IteratorRandom;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let system = sysinfo::System::new();
    let mut rng = rand::thread_rng();

    let proc = system
        .get_process_list()
        .iter()
        .filter(|(pid, _)| **pid >= 1024)
        .map(|(_, proc)| proc)
        .choose(&mut rng)
        .unwrap();

    println!("I'm angry, gonna kill {}", proc.name());
    thread::sleep(time::Duration::from_secs(5));
    proc.kill(sysinfo::Signal::Kill);
}
