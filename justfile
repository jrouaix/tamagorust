# Just manual: https://github.com/casey/just

_default:
	@just --list --unsorted

# build_tama:
#   cd tama && cargo +esp b --target=xtensa-esp32-none-elf -Zbuild-std=alloc,core --release

build_debug:
  cargo build -p tama

flash_debug: build_debug
  espflash flash --monitor target/xtensa-esp32-none-elf/debug/tama

# # Build the debug version of the firmware
# build_debug:
#   cargo build

# # Build the release version of the firmware
# build_release:
#   cargo build --release

# # Build, Flash & Monitor the debug firmware
# flash_mon: build_debug
#   espflash flash target/xtensa-esp32-espidf/debug/tama --monitor --list-all-ports

# # Build & Flash the debug firmware
# flash_debug: build_debug
#   espflash flash target/xtensa-esp32-espidf/debug/tama

# # Build & Flash the debug firmware with webflash
# webflash_debug: build_debug
#   web-flash --chip esp32 target/xtensa-esp32-espidf/debug/tama

# # Plug minicom to the device
# minicom:
#   minicom -D /dev/ttyUSB0 -b 115200

# Install the development environment
install_dev_env:
  cargo install espup
  espup install
  cargo install espflash


  # source $HOME/export-esp.sh
