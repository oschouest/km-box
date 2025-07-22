# KM-Box Project Log & Roadmap

## 🎯 Ultimate Vision: AI-Powered Gaming Enhancement System

### End-State Architecture:
```
Gaming PC ← OBS Stream ← AI Analysis PC (Aimmy)
    ↓                           ↓
USB HID Input              AI Decisions
    ↓                           ↓
Teensy 4.0 ← UART ← Raspberry Pi 5 ← AI Commands
    ↓                           ↓
KM-Box Relay            Recoil Compensation
    ↓                           ↓
Enhanced Gaming Input   Adaptive AI Logic
```

### Vision Components:
- **Gaming PC**: Runs game, streams to AI PC via OBS
- **AI Analysis PC**: Runs Aimmy AI for visual analysis & decision making
- **KM-Box System**: Hardware relay with AI-driven input modification
- **Adaptive Recoil**: OCR-based weapon detection & automatic compensation

---

## Phase 3 Complete - HID Input Capture ✅
**Date: July 22, 2025**

### Achievements:
- ✅ **Git-Based Workflow**: Established push/pull sync with GitHub
- ✅ **HID Input Capture**: 99 mouse reports captured via hidapi
- ✅ **Device Detection**: SteelSeries mouse (VID=1038, PID=183a) working
- ✅ **Clean Workspace**: Removed 5 duplicate main_* files
- ✅ **Build System**: `sync-pi.sh` automated build/deploy
- ✅ **UART Ready**: Pi GPIO 14/15 ↔ Teensy pins 0/1 configured

### Technical Implementation:
```rust
// Working HID capture in main.rs
let device = api.open(mouse_vid, mouse_pid)?;
loop {
    match device.read_timeout(&mut buf, 100) {
        Ok(size) if size > 0 => {
            let hex_data = hex::encode(&buf[..size]);
            let uart_msg = format!("HID:{}\n", hex_data);
            uart_port.write_all(uart_msg.as_bytes())?;
        }
    }
}
```

---

## Phase 2 Complete - UART Communication ✅
**Date: July 21, 2025**

### Achievements:
- ✅ Pi↔Teensy UART communication established
- ✅ GPIO pin configuration (Pi GPIO 14/15)
- ✅ Teensy Serial1 configuration (pins 0/1)
- ✅ Bidirectional message exchange verified

---

## Phase 1 Complete - Environment Setup ✅
**Date: July 21, 2025**

### Achievements:
- ✅ SSH key-based authentication (pi5 alias)
- ✅ Rust toolchain on Pi (1.88.0)
- ✅ PlatformIO for Teensy development
- ✅ Git repository initialization
- ✅ Project structure creation

---

# 🚀 FUTURE DEVELOPMENT ROADMAP

## Phase 4: USB HID Output (Current Sprint)
**Target:** Complete basic relay functionality
- [ ] Update Teensy firmware to parse "HID:..." UART messages
- [ ] Decode hex data back to raw HID reports
- [ ] Output via USB HID (Mouse.h/Keyboard.h)
- [ ] Test full Pi→Teensy→PC relay chain
- [ ] Latency benchmarking

## Phase 5: Input Modification Framework
**Target:** Add input transformation capabilities
- [ ] Pi-side input filtering and modification logic
- [ ] Configurable input mappings (JSON/TOML config)
- [ ] Real-time parameter adjustment via commands
- [ ] Mouse sensitivity scaling
- [ ] Key remapping and macro support

## Phase 6: Recoil Compensation Engine
**Target:** Game-specific recoil scripts for Rainbow Six Siege
- [ ] Weapon profile system (JSON configurations)
- [ ] Recoil pattern database for R6S weapons
- [ ] Automatic pattern detection and compensation
- [ ] Timing-based recoil control
- [ ] Attachment-specific pattern variations
- [ ] **Extend to other games**: CSGO, Valorant, Apex Legends

## Phase 7: Network Integration
**Target:** Remote control and AI command interface
- [ ] TCP/UDP server on Pi for external commands
- [ ] REST API for real-time parameter adjustment
- [ ] WebSocket interface for live monitoring
- [ ] Command protocol for external AI systems
- [ ] Secure authentication and encryption

## Phase 8: AI Integration Foundation
**Target:** Aimmy AI integration architecture
- [ ] **Aimmy GitHub Integration**: Fork and study codebase
- [ ] Network protocol design for AI→KM-Box communication
- [ ] OBS streaming setup documentation
- [ ] AI command parsing and validation
- [ ] Fail-safe and manual override systems

## Phase 9: Adaptive Recoil Intelligence
**Target:** OCR-based automatic weapon detection
- [ ] **OCR Integration**: Weapon/attachment detection from game UI
- [ ] **Dynamic Profile Loading**: Auto-switch recoil patterns
- [ ] **Attachment Recognition**: Compensator, muzzle brake, etc.
- [ ] **Real-time Adaptation**: Learn and adjust to gameplay
- [ ] **Multi-game Support**: Generic weapon detection system

## Phase 10: Full AI Enhancement System
**Target:** Complete Aimmy AI integration
- [ ] **Multi-PC Architecture**: Gaming PC + AI Analysis PC
- [ ] **OBS Stream Processing**: Real-time game analysis
- [ ] **AI Decision Engine**: Visual analysis → Input modifications
- [ ] **Adaptive Learning**: Improve performance over time
- [ ] **Advanced Features**: Predictive aiming, movement optimization

## Phase 11: Production Optimization
**Target:** Performance and reliability
- [ ] **Sub-millisecond Latency**: Hardware and software optimization
- [ ] **Anti-detection**: Ensure undetectable operation
- [ ] **Stability**: 24/7 operation capability
- [ ] **Monitoring**: Performance metrics and alerting
- [ ] **Update System**: Remote firmware/software updates

## Phase 12: Ecosystem Development
**Target:** Community and extensibility
- [ ] **Plugin Architecture**: Custom game support
- [ ] **Profile Sharing**: Community recoil patterns
- [ ] **GUI Configuration Tool**: User-friendly setup
- [ ] **Mobile App**: Remote monitoring and control
- [ ] **Documentation**: Complete developer and user guides

---

# 📋 Technical Specifications

## Communication Protocols:
- **Pi↔Teensy**: UART, 9600 baud, hex-encoded HID reports
- **AI↔Pi**: TCP/WebSocket, JSON command format
- **Gaming PC↔AI PC**: OBS streaming, network commands

## Performance Targets:
- **Input Latency**: <1ms Pi→Teensy relay
- **AI Processing**: <5ms analysis-to-command
- **Total System**: <10ms input-to-output
- **Reliability**: 99.9% uptime, error recovery

## Supported Games (Planned):
1. **Rainbow Six Siege** (Priority 1)
2. **Counter-Strike 2** 
3. **Valorant**
4. **Apex Legends**
5. **Call of Duty series**
6. **Generic FPS support**

## Hardware Requirements:
- **KM-Box**: Raspberry Pi 5 + Teensy 4.0
- **AI PC**: RTX 4060+ for real-time OCR/analysis
- **Gaming PC**: High-refresh monitor, stable FPS
- **Network**: Low-latency LAN between PCs
