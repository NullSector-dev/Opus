default:
	@just --list

build:
	@echo "Building Opus...."
	@cargo build --release
	@echo "Build complete procceed to Installation!"

install:
	@echo "Initiating Build process"
	@cargo build --release
	@echo "Build complete!"
	@echo "Installing Opus...."
	@sudo cp ./target/release/opus /usr/local/bin
	@echo "cleaning build artifacts...."
	@cargo clean
	@echo "Build Artifacts has been cleaned!"
	@echo "Opus has been installed successfully!"

clean:
	@echo "cleaning Build Artifacts...."
	@cargo clean
	@echo "Cleaned Build Artifacts!"

uninstall:
	@echo "Uninstalling Opus...."
	@rm -rf /usr/local/bin/opus
	@echo "Uninstalled Opus successfully!"
	
