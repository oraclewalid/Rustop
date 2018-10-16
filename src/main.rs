extern crate rand;
extern crate sysinfo;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use sysinfo::{NetworkExt, System, SystemExt};
fn main() {
	// We display the disks:
	
	let mut sys = System::new();
	println!("=> disk list:");
	for disk in sys.get_disks() {
	    println!("{:?}", disk);
	}

	// Network data:
	println!("input data : {} B", sys.get_network().get_income());
	println!("output data: {} B", sys.get_network().get_outcome());

	// Components temperature:
	for component in sys.get_components_list() {
	    println!("{:?}", component);
	}

	// Memory information:
	println!("total memory: {} kB", sys.get_total_memory());
	println!("used memory : {} kB", sys.get_used_memory());
	println!("total swap  : {} kB", sys.get_total_swap());
	println!("used swap   : {} kB", sys.get_used_swap());

	// Number of processes
	println!("NB processes: {}", sys.get_processor_list().len());

	for process in sys.get_processor_list() {
	    println!("{:?}", process);
	}

	// To refresh all system information:
	sys.refresh_all();
}

fn old() {

  
    loop {
    	 let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");

    	let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => continue,
		};
		let random = rand::thread_rng().gen_range(1, 101);
	    match random.cmp(&guess) {
	    	Ordering::Less => println!("small"),
	    	Ordering::Greater => println!("big"),
	    	Ordering::Equal=> println!("Equal"),
	    }
    }
}