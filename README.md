# BF-Arduino

A Rust Arduino Brainfuck interpreter. Uses the [Arduino](https://github.com/avr-rust/arduino/) library to make code simpler. Uses copy-pasted code.

## Compiling
You will need to compile a working version of [`rustc` for AVR](https://github.com/avr-rust/rust). [@gergoerdi](https://github.com/gergoerdi) wrote a [nice tutorial](https://github.com/gergoerdi/rust-avr-chip8-avr/blob/master/README.md) for this. These steps do work under WSL (Windows Subsystem for Linux).

## Flashing The Hex
You can use [avrdude](http://www.nongnu.org/avrdude/) to flash the hex file to the AVR. I found [this](https://typeunsafe.wordpress.com/2011/07/22/programming-arduino-with-avrdude/) guide and [this](https://learn.sparkfun.com/tutorials/pocket-avr-programmer-hookup-guide/using-avrdude) guide useful.

### Example:

```
avrdude -v -p atmega328p -c arduino -P /dev/ttyUSB0 -b 57600 -D -U flash:w:target/arduino/release/bf_arduino.hex:i
```

You can replace `/dev/ttyUSB0` with your port connected to the AVR, e.g `/dev/ttyUSBX` or, under windows `COMX`, where `X` is your port number. A quick way to find out which port it is plugged into is to use the [Arduino IDE](https://www.arduino.cc/en/Main/Software), and find it under `Tools > Port`.
