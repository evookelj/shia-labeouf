use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use matrix::Gmatrix;
use display::disp;
use display::save_ppm;
use draw::draw_lines;

pub fn parse_file(name: &str, transf: &mut Gmatrix, edges: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500]) {
	let f = File::open(name).unwrap();
	let mut file = BufReader::new(&f);
	let mut next: String = "".to_string();
	let mut l;
	let mut transf = Gmatrix::new();
	let line = String::from("line");
	let stuff = [
		String::from("line"),
		String::from("ident")
	]
	for line in file.lines() {
		l = line.unwrap();

		match next {
			line => {
				let mut split = l.split(" ");
				let vec: Vec<&str> = split.collect();
				edges.add_edge(vec[0].parse().unwrap(), 
					vec[1].parse().unwrap(), 
					vec[2].parse().unwrap(), 
					vec[3].parse().unwrap());
				next="".to_string();
			}

			"ident" => {
				transf = edges.identity();
			}

			"save" => {
				println!("saving");
				save_ppm(screen,&l)

			}

			"" => {
				if l=="display" {
					println!("displaying");
					println!("clen {}", edges.clen());
					draw_lines(edges,screen,[255,255,255]);
					disp(screen);
				}
				next = l.to_string();
			}
			_ => ()
		}
	}
}