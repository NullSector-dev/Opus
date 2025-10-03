default:
	@just --list

build:
	@echo "Building Opus...."
	@cargo build --release
	@echo "Build complete procceed to Installation!"

install:
	@echo "Installing Opus...."
	@cp ./target/release/opus /usr/local/bin
	@echo "Opus has been installed successfully!"
	@echo "It is encoraged to clean artifacts post installation."

clean:
	@echo "cleaning Build Artifacts...."
	@cargo clean
	@echo "Cleaned Build Artifacts!"

uninstall:
	@echo "Uninstalling Opus...."
	@rm -rf /usr/local/bin/opus
	@echo "Uninstalled Opus successfully!"
	
