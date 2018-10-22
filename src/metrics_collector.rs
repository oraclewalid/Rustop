use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};
use super::event::Event;

pub struct MetricsCollector {
    channel: Sender<Event>,
    sys: System,
}

impl MetricsCollector {

    pub fn new(channel: Sender<Event>, sys: System) -> MetricsCollector {
        return MetricsCollector { channel, sys };
    }

    pub fn run(&self) -> (){

            let tx1 = Sender::clone(&self.channel);
            thread::spawn(move || {
                match self.sys.cpu_load_aggregate() {
                    Ok(cpu)=> {
                        thread::sleep(Duration::from_secs(1));
                        let cpu = cpu.done().unwrap();
                        //let result = tx1.send(Event::CpuEvent(cpu.user * 100.0));
                        println!("CPU load: {}", cpu.user * 100.0);
                    },
                    Err(x) => println!("\nCPU load: error: {}", x)
                }
            });
    }
}