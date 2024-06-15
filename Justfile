# list available commands
l:
	@just --list


# run the generate all example
a:
	@cargo run --example generate_all

# build the example in release mode and run it
r:
	@cargo build --release --example generate_all
	strip target/release/examples/generate_all
	ls -lsa target/release/examples/generate_all
	target/release/examples/generate_all
