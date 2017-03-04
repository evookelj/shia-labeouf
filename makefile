main: main.rs
	rustc main.rs

run: main
	./main
	rm main
	display img.ppm
