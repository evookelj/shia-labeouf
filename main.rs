mod matrix;
mod display;
mod draw;

use matrix::Gmatrix;
use display::save_ppm;
use draw::draw_lines;
use display::disp;
use display::XRES;
use display::YRES;

fn main() {
	const XR: i32 = XRES as i32;
	const YR: i32 = YRES as i32;

	let mut s = [[[0; 3]; 500]; 500];
	let mut gm = Gmatrix::new();
	let mut i: i32 = 1;
	let mut j: i32 = YR-1;
	while i<XR {
		while j>0 {
			gm.add_edge(XR-i, j, YR-j, i);
			gm.add_edge(XR-i, YR-j, XR-j, YR-i);
			j-= 50;
		}
		j = YR-1;
		i+=10;
	}
	draw_lines(&mut gm, &mut s, [255,209,220]); //ffd1dc
	save_ppm(&mut s, "img.ppm");
	disp(s);
}