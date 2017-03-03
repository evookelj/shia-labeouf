use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

static XRES: usize = 500;
static YRES: usize = 500;
static MAXCOL: usize = 255;
static RED: usize = 0;
static GREEN: usize = 1;
static BLUE: usize = 2;

static DEFAULT: [u32; 3] = [0,0,0];

fn plot(screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3], x: usize, y:usize) {
	let newy = YRES-1-y;
	if x<XRES && newy < YRES {
		for i in 0..3 {
			println!("color[RED] {}", color[RED]);
			screen[newy][x][i] = color[i];
			println!("screen[RED] {}",screen[newy][x][RED] );
		}
	}
}

fn clear_screen(screen: &mut [[[u32; 3]; 500]; 500]) {
	for y in 0..YRES {
		for x in 0..XRES {
			screen[x as usize][y as usize] = DEFAULT;
		}
	}
}

fn save_ppm(screen: &mut [[[u32; 3]; 500]; 500], f: &str) {
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

fn display(screen: [[[u32; 3]; 500]; 500]) {
	    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}

fn main() {
	let mut screen = [[DEFAULT; 500]; 500];
	plot(&mut screen, [255,255,255],250,250);
	clear_screen(&mut screen);
	save_ppm(&mut screen, "img.ppm");
}