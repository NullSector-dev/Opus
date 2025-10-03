APP_NAME = opus
INSTALL_DIR = /usr/local/bin
BUILD_DIR = target/release

.PHONY: all build install uninstall clean

SUDO := $(shell command -v doas 2>/dev/null || echo sudo)

all: build

build:
	cargo build --release

install: 
	@echo "Installing Opus...."
	$(SUDO) cp $(BUILD_DIR)/$(APP_NAME) $(INSTALL_DIR)
	$(SUDO) chmod +x $(INSTALL_DIR)/$(APP_NAME)
	@echo "Opus has been installed successfully!"

uninstall:
	@echo "Uninstalling Opus...."
	$(SUDO) rm -rf $(INSTALL_DIR)/$(APP_NAME)
	@echo "Uninstalled Opus successfully"


clean:
	cargo clean
	@echo "Build artifacts removed"
