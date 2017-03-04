mod matrix;
mod display;
mod draw;

use matrix::Gmatrix;
use display::save_ppm;
use draw::draw_lines;

fn main() {
	let mut s = [[[0; 3]; 500]; 500];
	let mut gm = Gmatrix::new();
	let mut i = 1;
	let mut j = 499;
	while i<500 {
		while j>0 {
			println!("{}>0 {}",j, j>0);
			//gm.add_edge(i, j, j, i);
			gm.add_edge(500-i, j, 500-j, i);
			gm.add_edge(500-i, 500-j, 500-j, 500-i);
			j-= 50;
			println!("done");
		}
		j = 499;
		i+=10;
		println!("i {}", i);
	}
	draw_lines(&mut gm, &mut s, [255,255,255]);
	save_ppm(&mut s, "img.ppm");
}