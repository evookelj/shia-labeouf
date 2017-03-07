mod matrix;
mod display;
mod draw;
mod parser;

use matrix::Gmatrix;
use parser::parse_file;

fn main() {
	let mut s = [[[0; 3]; 500]; 500];
	let mut gm = Gmatrix::new();
	let mut shia = gm.identity();
	parse_file("script", &mut shia, &mut gm, &mut s);
}