

build_debug:
    ./scripts/build.sh "debug"

build_release:
    ./scripts/build.sh

flash_debug: build_debug
    espflash flash target/xtensa-esp32-espidf/debug/tama

webflash_debug: build_debug
    web-flash --chip esp32 target/xtensa-esp32-espidf/debug/tama