extern crate failure;
extern crate termion;
extern crate tui;
mod ui;
mod event_handler;


use std::sync::mpsc;
use event_handler::Event;
use std::thread;

fn main() -> Result<(), failure::Error> {


	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		let x = tx.send(Event::CpuEvent());
	});
	ui::launch_ui()
}