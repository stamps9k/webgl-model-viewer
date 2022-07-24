www: webasm-build
	npm install -C www

webasm-build:
	wasm-pack build webasm/

