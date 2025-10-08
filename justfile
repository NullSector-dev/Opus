default:
	@just --list

build:
	@echo "Initiating Build Process!"
	@echo "Building Opus...."
	@cargo build --release
	@echo "Build complete procceed to Installation!"

install:build
	@echo "Installing Opus...."
	@sudo cp ./target/release/opus /usr/local/bin/
	@just clean
	@echo "Opus has been installed successfully!"

clean:
	@echo "Cleaning Build Artifacts...."
	@cargo clean
	@echo "Cleaned Build Artifacts!"

reinstall:
	@echo "Reinstalling Opus!"
	@echo "Deleting bin file from /usr/local/bin ...."
	@sudo rm -rf /usr/local/bin/opus
	@echo "Deleted bin file!"
	@just build
	@echo "Reinstallling the bin files"
	@sudo cp ./target/release/opus /usr/local/bin/
	@just clean
	@echo "Opus has been reinstalled successfully!"

uninstall:
	@echo "Uninstalling Opus...."
	@sudo rm -rf /usr/local/bin/opus
	@echo "Uninstalled Opus successfully!"
	
