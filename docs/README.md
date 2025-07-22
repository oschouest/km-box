# KM-Box Documentation

## Phase 3: Input Capture Implementation

### Overview
Phase 3 introduces real-time input capture from keyboard and mouse devices connected to the Raspberry Pi, with events transmitted via UART to the Teensy for processing.

### Architecture
```
Input Devices → Pi (evdev) → UART → Teensy (event parser) → [Phase 4: USB HID]
```

### Dependencies Added
- **evdev v0.12.2**: Linux input device access
- **tokio v1.46.1**: Async runtime for non-blocking I/O

### Event Protocol
| Event Type | Format | Example | Description |
|------------|--------|---------|-------------|
| Key Press | `key:KEY_NAME:1` | `key:KEY_A:1` | Key pressed down |
| Key Release | `key:KEY_NAME:0` | `key:KEY_A:0` | Key released |
| Mouse X | `mouse:REL_X:value` | `mouse:REL_X:5` | Mouse moved right (+) or left (-) |
| Mouse Y | `mouse:REL_Y:value` | `mouse:REL_Y:-3` | Mouse moved up (-) or down (+) |

### Device Discovery
The system automatically discovers input devices by:
1. Scanning `/dev/input/event*` devices
2. Filtering for devices containing "keyboard", "mouse", "trackpad", or "touchpad"
3. Opening the first available device for monitoring

### Running Phase 3
```bash
# On Pi (with elevated permissions for input device access)
cd ~/km_box_project/pi_code
sudo ./target/release/km_pi
```

### Expected Output
```
=== KM-Box Phase 3: Input Capture & UART Relay ===
Initializing evdev input capture and UART communication...

✓ UART connected to Teensy at 9600 baud
✓ Sent initialization signal to Teensy
✓ Found 2 input device(s)
  - /dev/input/event0: USB Optical Mouse
  - /dev/input/event1: Dell KB216 Wired Keyboard

🎯 Starting input capture loop...
Press keys or move mouse - events will be sent to Teensy
Press Ctrl+C to stop

📡 Monitoring: /dev/input/event1
📤 key:KEY_H:1
📤 key:KEY_H:0
📤 key:KEY_E:1
📤 key:KEY_E:0
```

### Teensy Response
```
[HEARTBEAT] Phase 3 active - awaiting input events
[UART] Received: 'phase3_start'
[UART] Phase 3 initialization complete
[INPUT] Key KEY_H PRESSED
[INPUT] Key KEY_H RELEASED
[INPUT] Key KEY_E PRESSED
[INPUT] Key KEY_E RELEASED
```

### Troubleshooting
- **No devices found**: Ensure keyboard/mouse are connected to Pi USB ports
- **Permission denied**: Run with `sudo` for `/dev/input/` access
- **UART errors**: Verify GPIO 14/15 ↔ Teensy pins 0/1 wiring

### Next Phase
Phase 4 will implement USB HID output on the Teensy to relay captured events to the target PC.
