# KM-Box Project

A multi-platform project for keyboard/mouse passthrough using Teensy 4.0 and Raspberry Pi.

## Project Structure

- `teensy_fw/` - PlatformIO project for Teensy 4.0 firmware
- `pi_passthrough/` - Python scripts for Raspberry Pi
- `km-box.code-workspace` - VS Code workspace configuration

## Hardware Setup

### Teensy 4.0
- Connect Teensy 4.0 to target computer via USB
- Connect Teensy Serial1 pins to Raspberry Pi UART:
  - Teensy Pin 0 (RX1) → Pi GPIO 14 (TX)
  - Teensy Pin 1 (TX1) → Pi GPIO 15 (RX)
  - Connect GND between devices

### Raspberry Pi
- Enable UART in `/boot/config.txt`: `enable_uart=1`
- Disable serial console in `/boot/cmdline.txt`
- Install Python dependencies: `pip install -r requirements.txt`

## Usage

1. Flash the Teensy firmware using PlatformIO
2. Run the Python script on Raspberry Pi: `python send_serial_test.py`
3. Characters sent from Pi will be typed on the target computer

## Development

Open `km-box.code-workspace` in VS Code for the best development experience with:
- PlatformIO IDE extension for Teensy development
- Python extension for Pi scripts
- IntelliSense and debugging support
