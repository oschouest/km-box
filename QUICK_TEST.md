# KM-Box Testing Commands

## Quick Test Commands for Pi

### 1. Build the project
cd ~/km-box/pi_code
cargo build --release

### 2. List all HID devices 
sudo ./target/release/km_pi --list-devices

### 3. Run with SteelSeries mouse (default)
sudo ./target/release/km_pi --verbose

### 4. Test with custom mouse VID/PID
sudo ./target/release/km_pi --mouse-vid 0x1234 --mouse-pid 0x5678 --verbose

## Expected Results:
- Device discovery should find your mouse/keyboard
- Mouse movements → MOUSE:<hex> messages
- Keyboard presses → KEYBOARD:<hex> messages  
- Messages sent to Teensy via UART at 9600 baud

## If issues:
- Check sudo permissions
- Verify HID device paths with --list-devices
- Check hardware wiring (Pi GPIO 14/15 ↔ Teensy 0/1, GND)
