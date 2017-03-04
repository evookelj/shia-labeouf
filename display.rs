use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub const XRES: usize = 500;
pub const YRES: usize = 500;
pub const DEFAULT: [u32; 3] = [0,0,0];

pub fn plot(x: i32, y:i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	if x>(XRES as i32)-1 || y>(YRES as i32)-1 {
		println!("skip");
		return ;
	}
	let newy = YRES-(y as usize)-1;
	if (x as usize)<XRES && newy < YRES {
		for i in 0..3 {
			screen[newy][x as usize][i] = color[i];
		}
	}
}

pub fn clear_screen(screen: &mut [[[u32; 3]; 500]; 500]) {
	for y in 0..YRES {
		for x in 0..XRES {
			screen[x as usize][y as usize] = DEFAULT;
		}
	}
}

pub fn save_ppm(screen: &mut [[[u32; 3]; 500]; 500], f: &str) {
	static HEADER: &'static str = "P3\n500 500 255\n";
	let path = Path::new(f);
	let display = path.display();

	//create file
	let mut file = match File::create(path) {
        Err(why) => panic!("Error creating {} because {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

	//write header to file
	match file.write_all(HEADER.as_bytes()) {
		Err(why) => panic!("Error writing header because {}", why.description()),
		Ok(_) => (),
	};
	for y in 0..YRES {
		for x in 0..XRES {
			match file.write_all(format!("{} {} {}\n",screen[x][y][0],screen[x][y][1],screen[x][y][2]).as_bytes()) {
				Err(why) => panic!("Error writing pixel {} {} because {}", x, y, why.description()),
				Ok(_) => (),
			};
		}
	}
}

pub fn disp(screen: [[[u32; 3]; 500]; 500]) {
	let output = Command::new("sh")
                     .arg("-c")
                     .arg("echo hello")
                     .output()
                     .expect("failed to execute process");

	let hello = output.stdout;
}