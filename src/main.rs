use std::{thread, time};

use rand;
use rand::seq::IteratorRandom;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_processes();
    let mut rng = rand::thread_rng();

    let proc = system
        .processes()
        .iter()
        .filter(|(pid, _)| **pid >= Pid::from_u32(1024))
        .map(|(_, proc)| proc)
        .choose(&mut rng)
        .unwrap();

    println!("I'm angry, gonna kill {}", proc.name());
    thread::sleep(time::Duration::from_secs(5));
    proc.kill();
}
