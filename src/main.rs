extern crate systemstat;

use systemstat::{System, Platform};
use std::sync::mpsc;

mod metrics_collector;
mod event;
fn main() {
	let platform= System::new();
	let (tx, rx) = mpsc::channel();
	let metrics = metrics_collector::MetricsCollector::new(tx, platform);

	metrics.run()
}