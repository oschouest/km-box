# Teensy Firmware

This directory contains the PlatformIO project for the Teensy 4.0 firmware.

## Features

- Reads characters from Serial1 (connected to Raspberry Pi)
- Types received characters using USB HID keyboard
- Debug output via USB Serial

## Building and Uploading

1. Open this folder in VS Code with PlatformIO extension
2. Build: `Ctrl+Alt+B` or use PlatformIO: Build
3. Upload: `Ctrl+Alt+U` or use PlatformIO: Upload

## Hardware Connections

- Pin 0 (RX1) - Connect to Pi GPIO 14 (TX)
- Pin 1 (TX1) - Connect to Pi GPIO 15 (RX)
- GND - Connect to Pi GND
