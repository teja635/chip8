use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::process;

pub fn assemble(program_file: &str, output_binary: &str) {
	let fd = File::open(program_file).unwrap();
	let f = BufReader::new(fd);
	
	for instruction_line in f.lines() {
		let line = instruction_line.unwrap();
		let instr: Vec<_> = line.split(" ").collect();
		match instr[0] {
			"CLS" => {
				println!("[+] Clearing display");
			},
			"RET" => {
				println!("[+] Returning from function");
			}
			"JP" => {
				println!("[+] Jump to address");
			},
			"CALL" => {
				println!("[+] Calling a function");
			},
			_ => { 
				eprintln!("[-] Assembler has not implemented instruction {} yet", instr[0]);
				process::exit(1);
			}
		}
	}
}
