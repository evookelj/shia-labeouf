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
	let mut i: i32 = YR/4;
	let mut j: i32 = (XR-1)*3/4;
	while i<(3*YR)/4 {
		while j>(XR-1)/4 {
			gm.add_edge(YR-i, j, XR-j, i);
			gm.add_edge(YR-i, XR-j, YR-j, XR-i);
			j-= 50;
		}
		j = (XR-1)*3/4;
		i+=10;
	}

	let scale = gm.make_scale(0.8,0.8,0.8);
	println!("SCALE MATRIX:");
	scale.print();
	let gs = scale.m_mult(gm);

	let trans = gs.make_trans(-50.0,-50.0,1.0);
	println!("TRANSLATE MATRIX:");
	trans.print();
	let mut gt = trans.m_mult(gs);

	let rot = gt.make_rotZ(30.0);
	println!("ROTZ MATRIX:");
	rot.print();
	let mut gr = rot.m_mult(gt);

	let mut fin = &mut gr;
	draw_lines(&mut fin, &mut s, [255,209,220]); //ffd1dc
	save_ppm(&mut s, "img.ppm");
	disp(&mut s);
}