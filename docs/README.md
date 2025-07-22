# KM-Box: Low-Latency Gaming Input Relay

## 🎯 Project Overview
A high-performance keyboard/mouse pass-through system for gaming, featuring:
- **Raspberry Pi 5**: HID input capture & processing (Rust)
- **Teensy 4.0**: USB HID output to gaming PC (C++)
- **UART**: Low-latency communication between devices

## 🏗️ Architecture
```
Input Devices → Pi (hidapi) → UART → Teensy 4.0 → Gaming PC
     USB             Rust        GPIO        C++        USB HID
```

## ✅ Current Status: Phase 3 Complete

### Working Components:
- **HID Input Capture**: 99 mouse reports captured via hidapi
- **Device Detection**: SteelSeries mouse (VID=1038, PID=183a)
- **Git Workflow**: Push/pull sync with automated build
- **UART Ready**: Pi GPIO 14/15 ↔ Teensy pins 0/1

### Binaries:
- `km_pi`: Full HID→UART relay system
- `hid_test`: Validation and testing utility

## 🚀 Quick Start

### Development Workflow:
```powershell
# 1. Edit code locally in VS Code
# 2. Commit and push to GitHub
git add .; git commit -m "changes"; git push

# 3. Sync and build on Pi
ssh pi5 "cd ~/km-box && ./sync-pi.sh"

# 4. Test HID capture
ssh pi5 "cd ~/km-box && sudo ./pi_code/target/release/hid_test"
```

## 📋 Phase Progress

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Environment Setup | ✅ Complete |
| 2 | UART Communication | ✅ Complete |
| 3 | HID Input Capture | ✅ Complete |
| 4 | USB HID Output | ⏳ Next |
| 5 | Full Relay System | ⏳ Pending |
| 6 | Optimization | ⏳ Pending |

## 🛠️ Technical Stack

### Rust (Pi):
- `hidapi`: HID device access
- `serialport`: UART communication
- `clap`: Command-line interface
- `log`/`env_logger`: Logging

### C++ (Teensy):
- Arduino framework
- `Mouse.h`/`Keyboard.h`: USB HID output
- `HardwareSerial`: UART communication

## 📡 Communication Protocol
```
Pi → Teensy: "HID:001234abcd\n"
Format: "HID:" + hex_encoded_report + "\n"
```

## 🔧 Hardware Setup
- **Pi GPIO 14/15** ↔ **Teensy pins 0/1** (UART)
- **Common GND** connection required
- **USB**: Input devices → Pi, Teensy → Gaming PC

## 📁 Project Structure
```
km-box/
├── pi_code/           # Rust project (HID capture)
│   ├── src/main.rs    # Full relay system
│   └── src/hid_test.rs # Test utility
├── teensy_fw/         # PlatformIO project
├── docs/              # Documentation
└── sync-pi.sh         # Build/deploy script
```

## 🎮 Next: Phase 4
Implement USB HID output on Teensy to complete the input relay chain.
