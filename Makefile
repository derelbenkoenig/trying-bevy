.PHONY: build run

# excluding the rendering feature for now because it complains on my WSL
# ok that doesn't work either actually
build:
	cargo build

run:
	cargo run
