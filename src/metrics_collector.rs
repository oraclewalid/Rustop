use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use super::event::Event;
use systemstat::{Platform, System};

pub struct MetricsCollector {
    pub channel: Sender<Event>,
    pub sys: System,
}

impl MetricsCollector {

    pub fn new(channel: Sender<Event>, sys: System) -> MetricsCollector {
        return MetricsCollector { channel, sys };
    }

    pub fn run(&self) -> () {
        match self.sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                let result = self.channel.send(Event::CpuEvent(cpu.user * 100.0));
                println!("CPU load: {}", cpu.user * 100.0);
            }
            Err(x) => println!("CPU load: error: {}", x)
        }
    }
}