use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;
use matrix::Gmatrix;
use display::disp;
use display::clear_screen;
use display::save_ppm;
use draw::draw_lines;

pub fn parse_file(name: &str, transf: &mut Gmatrix, edges: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500]) {
	let f = File::open(name).unwrap();
	let file = BufReader::new(&f);
	let mut last = String::from("");
	let mut l: String;
	for line in file.lines() {
		l = line.unwrap();

		let split = l.split(" ");
 		let vec: Vec<&str> = split.collect();

		match last.trim() {
			"save" => {
				draw_lines(edges, screen, [255,255,255]);
				println!("Saving as {}..", vec[0]);
				save_ppm(screen, vec[0]);
			 	clear_screen(screen);
			 	last = String::from("");
			 }
			"line" => {
 				edges.add_edge(vec[0].parse().unwrap(), 
 					vec[1].parse().unwrap(), 
 					vec[2].parse().unwrap(),
 					vec[3].parse().unwrap(), 
 					vec[4].parse().unwrap(),
 					vec[5].parse().unwrap());
 				last = String::from("");
			}
			"scale" => {
				let scale = edges.make_scale(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
					);
				scale.edit_mult(transf);
				last = String::from("");
			}
			"move" => {
				let trans = edges.make_trans(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
				);
				trans.edit_mult(transf);
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
				rot.edit_mult(transf);
				last = String::from("");
			}
			_ => {
				match l.trim() {
				"ident" => {
					let g = edges.identity();
					for i in 0..g.rlen() {
						for c in 0..g.clen() {
							transf.set_val(i,c,g.get_val(i,c));
						}
					}
				}
				"apply" => transf.edit_mult(edges),
				"display" => {
					draw_lines(edges, screen, [255,255,255]);
					disp(screen);
					clear_screen(screen);
				}
				_ => last = String::from(vec[0]),
				}
			}
		}
	}
}