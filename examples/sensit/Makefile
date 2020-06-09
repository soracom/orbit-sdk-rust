.PHONY: deps test clean build serve-local package

build: deps
	#cargo wasi build --release
	cargo build --release --target wasm32-unknown-unknown

clean:
	cargo clean