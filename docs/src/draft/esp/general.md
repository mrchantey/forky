# Embedded

## Quickstart
- [commands](../../crates/forky_esp/justfile)

## Generate from template
- `cargo generate --git https://github.com/esp-rs/esp-idf-template cargo`

## ESP32 C3

- [My Board](https://core-electronics.com.au/esp32-c3-mini-development-board.html)

1. [Board Drivers](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/get-started/establish-serial-connection.html)
   - Device may appear in `Device Manager -> Other Devices -> USB to UART ...`
   - Visit `https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers?tab=downloads`
		- Universal Windows Driver -> `silabser.inf`->right click->install
   - Visit `https://ftdichip.com/drivers/vcp-drivers/`
		- Windows Driver (both VCP & D2XX)
	- Device should appear in `Device Manager -> Ports (COM & LPT) -> Silicon Labs CP210x USB to UART Bridge (COMX)`
1. Rust Board connection
   ```sh
	cargo install cargo-espflash
	cargo install espmonitor
	# cargo install cargo-espmonitor # not required?
	cargo espflash board-info
	cargo esmonitor COM3
	 ```
1. Install espup - clang, idf etc
	```sh
	cargo install espup --git https://github.com/esp-rs/espup
	espup install # takes 10-20 mins
	./export-esp.ps1
	```
1. Install dependencies
	```sh
	rustup toolchain install nightly --component rust-src
	rustup default nightly
	# cargo install ldproxy #handled by espup?
	```
2. (Optional) Create idf project
   ```sh
	cargo install cargo-generate
	cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
	cd project-name
	cargo run build
	cargo espflash COM3 target/riscv32imc-esp-espidf/debug/esp-template
	cargo espmonitor
	```

# LEDS
[comparison](https://www.stripsledlight.com/what-different-of-apa102sk9822hd107sws2812b-sk6812ws2811ws2815ws2813/#:~:text=Apa102c%20is%20the%20same%20as,led%20chip%2C%20sometimes%20have%20a)
- WS2812
- SK6812
- APA102
	- Fast

# MPU6050

- https://docs.rs/mpu6050/latest/mpu6050/

## Resources
- [Ferrous Systems Rust](https://espressif-trainings.ferrous-systems.com/01_intro.html)
- [ESP32-C3 HAL](https://github.com/esp-rs/esp-hal/tree/main/esp32c3-hal)
- [Rust Book](https://esp-rs.github.io/book/overview/using-the-standard-library.html)
