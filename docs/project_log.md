# Project Log

## Phase 1 Setup - Initial Environment Setup

### Date: July 21, 2025

#### Tasks Completed:
- ✅ SSH key setup initiated (4096-bit RSA key generated)
- ✅ Folders created: pi_code, teensy_code, docs
- ✅ Git initialization completed
- ✅ Teensy PlatformIO project structure preserved and copied to teensy_code
- ✅ SSH key-based authentication setup (key copied to Pi successfully)
- ✅ SSH connection tested and working without password
- ✅ Rust installation on Pi (Rust 1.88.0 installed successfully)
- ✅ Cargo project creation in pi_code (km_pi project created and tested)
- ✅ Teensy PlatformIO build test (compiled successfully, 4.21 seconds)
- ✅ Basic "Hello World" validation on both platforms

#### Current Structure:
```
km-box/
├── pi_code/           (for Rust project)
├── teensy_code/       (PlatformIO project for Teensy 4.0)
│   ├── platformio.ini
│   ├── src/main.cpp
│   └── .vscode/
├── docs/
│   ├── project_log.md
│   └── copilot_instructions.md
└── README.md
```

#### Next Steps:
1. Complete SSH key setup by entering password 4512 when prompted
2. Update SSH config to include IdentityFile ~/.ssh/id_rsa
3. Test SSH connection to Pi without password
4. Install Rust on Pi via SSH
5. Create Cargo project in pi_code
6. Test basic compilation on both platforms

#### Notes:
- Old pi_passthrough folder will be cleaned up after VS Code releases file handles
- Teensy project already configured for Arduino framework
- Monitor speed set to 115200 in platformio.ini
- PlatformIO executable found at: ~/.platformio/penv/Scripts/platformio.exe

## Phase 1 Results - ✅ COMPLETED

### Build Test Results:
**Raspberry Pi (Rust):**
- ✅ Rust 1.88.0 installed successfully
- ✅ Cargo project "km_pi" created in ~/km_box_project/km_pi
- ✅ Compilation successful (0.27s)
- ✅ "Hello, world!" output confirmed

**Teensy 4.0 (C++ Arduino):**
- ✅ PlatformIO build successful (4.21 seconds)
- ✅ Memory usage: FLASH 21,500 bytes used, 2,010,116 bytes free
- ✅ Memory usage: RAM2 12,416 bytes used, 511,872 bytes free
- ✅ Ready for upload to hardware

### Environment Validation:
- ✅ SSH key-based authentication working (no password required)
- ✅ VS Code tasks configured for both platforms
- ✅ Build toolchains operational on both targets
- ✅ Project structure established and documented

## Post-Phase 1 Cleanup - July 22, 2025

#### Cleanup Tasks Completed:
- ✅ Moved documentation files from teensy_fw to docs/ folder
  - project_log.md → docs/project_log.md
  - copilot_instructions.md → docs/copilot_instructions.md  
  - PHASE_1_SUMMARY.md → docs/PHASE_1_SUMMARY.md
  - pi_code_reference.md → docs/pi_code_reference.md
- ✅ Updated root README.md with comprehensive project information
- ✅ Created main .gitignore file in root directory
- ⚠️ pi_passthrough folder deletion pending (locked by VS Code process)

#### Final Project Structure:
```
km-box/
├── .git/                  # Git repository
├── docs/                  # Documentation
│   ├── project_log.md
│   ├── copilot_instructions.md
│   ├── PHASE_1_SUMMARY.md
│   └── pi_code_reference.md
├── pi_code/               # Local Rust development (empty, develop on Pi)
├── teensy_code/           # Clean PlatformIO project for Teensy 4.0
├── teensy_fw/             # Original workspace (to be removed after cleanup)
├── pi_passthrough/        # Old Python project (locked, manual removal needed)
├── .gitignore
├── README.md              # Updated with full project info
└── km-box.code-workspace  # VS Code workspace configuration
```

#### Notes:
- pi_passthrough folder requires manual deletion after VS Code releases file handles
- teensy_fw can be removed after confirming all important files moved to docs/
- Project ready for Phase 2 development

## Fixed cwd error and Phase 2 setup - July 22, 2025

### Issue Resolution:
- ✅ Fixed terminal startup error by deleting problematic .vscode folder in teensy_code
- ✅ Updated km-box.code-workspace configuration for proper folder structure
- ✅ Created workspace-level .vscode/tasks.json with correct paths
- ✅ Terminal now starts properly in workspace root

### Phase 2 Implementation:
- ✅ Updated Teensy main.cpp for UART communication testing
- ✅ Added command processing: ping, led_on, led_off, test, status, heartbeat
- ✅ Implemented comprehensive debug output via USB Serial
- ✅ Build successful: 3.16 seconds, 17KB flash used, 485KB RAM free
- ✅ SSH connection to Pi verified and working

### Hardware Wiring Required:
- Pi GPIO 14 (TX) → Teensy RX1 (Pin 0)
- Pi GPIO 15 (RX) → Teensy TX1 (Pin 1)  
- Pi GND → Teensy GND

### IntelliSense Fixes:
- ✅ Fixed "cannot open source file Arduino.h" error
- ✅ Fixed "include errors detected. Please update your includePath" error
- ✅ Created c_cpp_properties.json with proper Teensy include paths
- ✅ Generated compile_commands.json for accurate IntelliSense
- ✅ Added settings.json for C++ configuration

### Ready for Testing:
- Teensy code ready for upload and UART communication
- Pi Rust UART sender code needed for full bidirectional test
- VS Code tasks configured: Build, Upload, Monitor, SSH tests
- IntelliSense fully working with no more red squiggles

## Phase 2 Results - ✅ COMPLETED

### Pi-side Implementation:
- ✅ SSH alias "pi5" configured and working (otis@192.168.1.117)
- ✅ Updated Rust code with proper serialport builder pattern (serialport = "4.2")
- ✅ Pi UART communication code implemented in main.rs
- ✅ Code successfully synced to Pi and built in release mode (0.37s)
- ✅ User "otis" confirmed in dialout group for serial access
- ✅ /dev/serial0 available and linked to ttyAMA10

### VS Code Configuration:
- ✅ Updated tasks.json with correct "pi5" SSH alias 
- ✅ Added comprehensive Pi development tasks:
  - "Sync Pi Code to Pi" (scp to Pi)
  - "Build Rust on Pi" (remote cargo build --release)
  - "Run Rust on Pi" (remote execution with sudo)
  - "Test SSH to Pi" (connection verification)

### Pi UART Code Features:
- ✅ Opens /dev/serial0 at 115200 baud using serialport builder
- ✅ Sends commands in sequence: ping, test, led_on, led_off, status
- ✅ Reads and displays Teensy responses with byte counts
- ✅ 2-second intervals between commands with proper error handling
- ✅ Clear hardware wiring instructions displayed on startup

### Build Status:
- **Pi Rust**: Build successful (0.37s) in release mode
- **Teensy C++**: Previous build successful (3.16s, 17KB flash)

### Next Steps:
1. Upload Teensy firmware: Use "Upload to Teensy" task
2. Start serial monitor: Use "Monitor Teensy Serial" task  
3. Connect hardware: Pi GPIO 14→Teensy Pin 0, Pi GPIO 15→Teensy Pin 1, GND→GND
4. Run Pi UART test: Use "Run Rust on Pi" task
5. Verify bidirectional communication in Serial Monitor

