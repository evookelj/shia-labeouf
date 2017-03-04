use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

static XRES: usize = 500;
static YRES: usize = 500;
static DEFAULT: [u32; 3] = [0,0,0];

pub fn plot(x: i32, y:i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	println!("y {}", y);
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