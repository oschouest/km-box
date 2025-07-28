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

## ✅ Current Status: Phase 5 COMPLETE - Perfect HID Passthrough Achieved

### Final Breakthrough: Complete Working Mouse Relay
- **Full int16 Support**: Handles complete mouse movement range (-32768 to +32767)
- **Perfect Fidelity**: No data loss from original mouse input to USB output
- **All Buttons Working**: Complete support for all mouse buttons and scroll wheel
- **Proper Speed**: 3.0x sensitivity for responsive mouse movement
- **No Click Conversion**: Fixed scroll wheel chunking bug

### Fully Working Components:
- **Input Modification Framework**: 3.0x sensitivity scaling on full int16 range ✅
- **Lossless HID Relay**: Pi → Teensy → PC with zero data truncation ✅
- **Button Processing**: Left/right/middle clicks with state tracking ✅
- **Scroll Wheel**: Fixed wheel event duplication, working perfectly ✅
- **Side Button Detection**: Back/Forward buttons detected and logged ✅
- **Configuration System**: TOML-based settings with CLI overrides ✅
- **Real-time Processing**: Live mouse input modification at full resolution ✅
- **Device Detection**: SteelSeries Aerox 3 (VID=1038, PID=183a) fully supported ✅
- **UART Communication**: 9-byte HID reports via 18-char hex encoding ✅
- **Teensy Firmware**: Chunked Mouse.move() with fixed wheel handling ✅

### Phase 5 Final Technical Achievement:
**HID Report Format Confirmed**:
```
9-byte format: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]
- Bytes 1-2: dx (little-endian int16) 
- Bytes 3-4: dy (little-endian int16)
- Byte 5: Buttons (0x01=L, 0x02=R, 0x04=M, 0x08=Back, 0x10=Forward)
- Byte 6: Wheel (signed int8)
```

**Final Testing Results**:
- Mouse movement: Fast and responsive (3x speed increase) ✅
- Button clicks: All working perfectly ✅
- Scroll wheel: Working properly (no click conversion) ✅
- Large movements: Preserved (dx=25, dy=14, etc.) ✅
- Side buttons: Detected and logged ✅

**Phase 5 Status: 100% COMPLETE** 🎉

## 🚀 Quick Start

### Development Workflow:
```powershell
# 1. Edit code locally in VS Code
# 2. Commit and push to GitHub
git add .; git commit -m "changes"; git push

# 3. Sync and build on Pi
ssh pi5 "cd ~/km-box && git pull"
ssh pi5 "source ~/.cargo/env && cd ~/km-box/pi_code && cargo build --release"

# 4. Test Phase 5 with sensitivity scaling
ssh pi5 "cd ~/km-box && sudo ./pi_code/target/release/km_pi --sensitivity 1.5 --verbose"

# 5. Test with button remapping
ssh pi5 "cd ~/km-box && sudo ./pi_code/target/release/km_pi --remap-buttons --verbose"
```

## 📋 Development Roadmap

| Phase | Description | Status | Focus |
|-------|-------------|--------|-------|
| 1-3 | Hardware Foundation | ✅ Complete | Pi + Teensy relay |
| 4 | USB HID Output | ✅ Complete | Basic relay functionality |
| 5 | Input Modification | ✅ Complete | **Sensitivity & button remap** |
| 6 | Recoil Scripts | ⏳ Next | **R6S weapon patterns** |
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
