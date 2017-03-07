use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use matrix::Gmatrix;
use display::disp;
use display::save_ppm;
use draw::draw_lines;
use std::string::String;

pub fn parse_file(name: &str, transf: &mut Gmatrix, edges: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500]) {
	let f = File::open(name).unwrap();
	let file = BufReader::new(&f);
	let mut last = String::new();
	let mut l: String;
	let mut transf = Gmatrix::new();
	for line in file.lines() {
		l = line.unwrap();
		println!("last {}\nthis {}\n", last, l);

		let split = l.split(" ");
 		let vec: Vec<&str> = split.collect();
		match last.trim() {
			"save" => {
				save_ppm(screen, vec[0]);
			 	last = String::from("");
			 	for y in 0..screen.len() {
						println!("clearing..");
						screen[y] = [[0; 3]; 500];
					}
			 }
			"line" => {
 				edges.add_edge(vec[0].parse().unwrap(), 
 					vec[1].parse().unwrap(), 
 					vec[2].parse().unwrap(), 
 					vec[3].parse().unwrap());
 				last = String::from("");
			}
			"scale" => {
				let scale = edges.make_scale(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
					);
				scale.edit_mult(&mut transf);
				last = String::from("");
			}
			"move" => {
				let trans = edges.make_trans(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
				);
				trans.edit_mult(&mut transf);
				last = String::from("");
			}
			"rotate" => {
				let mut rot = Gmatrix::new();
				match vec[0].trim() {
					"x" => rot = edges.make_rotX(vec[1].parse().unwrap()),
					"y" => rot = edges.make_rotY(vec[1].parse().unwrap()),
					"z" => rot = edges.make_rotZ(vec[1].parse().unwrap()),
					_ => ()
				}
				rot.edit_mult(&mut transf);
				last = String::from("");
			}
			_ => {
				match l.trim() {
				"ident" => {
					transf = edges.identity();
				}
				"apply" => transf.edit_mult(edges),
				"display" => {
					draw_lines(edges, screen, [255,209,220]);
					disp(screen);
				}
				_ => (),
				}
				last = String::from(vec[0]);
			}
		}
	}
}