# KM-Box: AI-Powered Gaming Enhancement System

## 🎯 Project Vision
An intelligent gaming input system that evolves from basic hardware relay to full AI-driven adaptive enhancement:

**Near-term**: Hardware relay + recoil compensation for Rainbow Six Siege  
**Long-term**: AI integration with Aimmy project for adaptive visual analysis

### Ultimate Architecture:
```
Gaming PC ← OBS Stream ← AI Analysis PC (Aimmy)
    ↓                           ↓
USB HID Input              AI Decisions
    ↓                           ↓
Teensy 4.0 ← UART ← Raspberry Pi 5 ← AI Commands
    ↓                           ↓
KM-Box Relay            Recoil Compensation
    ↓                           ↓
Enhanced Gaming         Adaptive AI Logic
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

## 📋 Development Roadmap

| Phase | Description | Status | Focus |
|-------|-------------|--------|-------|
| 1-3 | Hardware Foundation | ✅ Complete | Pi + Teensy relay |
| 4 | USB HID Output | ⏳ Next | Complete basic relay |
| 5 | Input Modification | ⏳ Pending | Filtering & transformation |
| 6 | Recoil Scripts | ⏳ Pending | **R6S weapon patterns** |
| 7 | Network API | ⏳ Pending | External control interface |
| 8 | AI Foundation | ⏳ Pending | **Aimmy integration prep** |
| 9 | Adaptive Recoil | ⏳ Pending | **OCR weapon detection** |
| 10 | Full AI System | ⏳ Pending | **Complete Aimmy integration** |
| 11 | Optimization | ⏳ Pending | <1ms latency, anti-detection |
| 12 | Ecosystem | ⏳ Pending | Community tools & profiles |

### Target Games:
1. **Rainbow Six Siege** (Priority 1 - recoil patterns)
2. Counter-Strike 2
3. Valorant  
4. Apex Legends
5. Call of Duty series

### AI Integration Goals:
- **Aimmy GitHub**: Fork and integrate visual analysis
- **Multi-PC Setup**: Gaming PC + AI Analysis PC  
- **OBS Streaming**: Real-time game analysis
- **Adaptive Logic**: Auto-adjust to weapon changes

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

## 🎮 Next Steps: Rainbow Six Siege Focus
**Immediate Priority**: Complete Phase 4, then implement R6S recoil compensation

**Future Vision**: OCR-based adaptive recoil that automatically detects weapons and attachments, eliminating the need for manual profile switching.

**AI Integration**: Work towards full Aimmy project integration for visual game analysis and intelligent input enhancement.
