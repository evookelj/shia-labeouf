main: main.rs
	rustc main.rs

run: main
	./main
	display img.ppm
	rm main
