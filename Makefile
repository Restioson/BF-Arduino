COMPILED:=target/arduino/release/bf_arduino.elf
HEX:=target/arduino/release/bf_arduino.hex

all: ${HEX}

.PHONY: ${COMPILED}
${COMPILED}:
	cargo build --release --target=./arduino.json --verbose

# Convert binary to an Intel HEX file for upload
${HEX}: ${COMPILED}
	avr-objcopy -O ihex -R .eeprom $< $@
