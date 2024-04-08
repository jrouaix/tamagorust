# tamagorust
Try embeded rust on m5 stick

## Dev environment setup

- install `rust` : https://www.rust-lang.org/tools/install
- then install `just` : `cargo install just`
- then install dependencies : `just install_dev_env`

Then open `vscode` with `source $HOME/export-esp.sh && code .` in this folder.

## Some commands

- `just`
- `screen /dev/ttyUSB0 115200` (Ctrl + A, K to exit)
- `minicom -D /dev/ttyUSB0 -b 115200` (Ctrl + A, X to exit)
- `ls /dev/ttyUSB*` (to check if device plugged, and find the correct port)


## Resources
- https://github.com/esp-rs/esp-hal
- https://github.com/esp-rs/esp-idf-template/
- cargo generate esp-rs/esp-idf-template cargo
- https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-connecting-wifi
- https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-button-controlled-blinking-by-timer-polling
- https://www.espressif.com/sites/default/files/documentation/esp32-pico_series_datasheet_en.pdf
- https://github.com/esp-rs/esp-idf-hal/tree/master/examples


## Steps
- [ ] Run embassy no-std
- [ ] Have a blinking led
- [ ] Have all buttons working
- [ ] Have the screen working
- [ ] Have sound working
- [ ] Have accelerometer working
- [ ] Build a Simon game
- [ ] ...