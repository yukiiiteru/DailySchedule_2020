all:
	rustc --crate-type=lib rary.rs
	rustc executable.rs --extern rary=library.rlib

ls:library.rlib
	ls lib*

run:executable
	./executable

clean:
	rm lib*
