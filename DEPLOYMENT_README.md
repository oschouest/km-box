README: Complete KM-Box async-hid Refactor

## What Changed
- Completely rewrote Pi code to use async-hid instead of evdev
- Much more reliable HID device access and event capture
- Added proper hex encoding for HID reports sent to Teensy
- Implemented VID/PID-based device targeting
- Added comprehensive logging and debugging options

## Deploy to Pi
1. Clone this repo to your Pi: `git clone <repo> ~/km-box`
2. Run: `bash ~/km-box/deploy-pi.sh`
3. Test with: `sudo ~/km-box/pi_code/target/release/km_pi --list-devices`

## Commands
- List devices: `sudo ./target/release/km_pi --list-devices`
- Run with verbose logging: `sudo ./target/release/km_pi --verbose`
- Custom mouse: `sudo ./target/release/km_pi --mouse-vid 0x1234 --mouse-pid 0x5678`

## What to Expect
- HID reports will be hex-encoded and sent to Teensy via UART
- Format: "MOUSE:<hex>" or "KEYBOARD:<hex>"
- Much cleaner than the old evdev approach!

Ready for testing!
