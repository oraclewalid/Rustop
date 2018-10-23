extern crate systemstat;

use metrics_collector::MetricsCollector;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use systemstat::{Platform, System};

mod metrics_collector;
mod event;

fn main() {
	let platform= System::new();
	let (tx, rx) = mpsc::channel();


	let _ = thread::spawn(move || {
		let metrics = MetricsCollector{channel : tx, sys: platform};
		metrics.run()
	});

	thread::sleep(Duration::from_secs(10));

}