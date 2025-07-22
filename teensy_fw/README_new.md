# KM Box - Low-Latency Keyboard/Mouse Pass-through System

## Project Overview
The KM Box is a hardware solution for low-latency keyboard and mouse pass-through and injection, designed for gaming applications. The system consists of a Raspberry Pi 5 connected to a Teensy 4.0 microcontroller via UART.

## Architecture
```
Gaming PC <-- USB HID --> Teensy 4.0 <-- UART --> Raspberry Pi 5 <-- USB --> Input Devices
```

## Hardware Components
- **Raspberry Pi 5**: Input capture and processing (Rust)
- **Teensy 4.0**: USB HID output to gaming PC (C++/Arduino)
- **UART Connection**: GPIO pins for Pi-Teensy communication

## Software Stack
- **Pi Code**: Rust with serialport, evdev, and tokio crates
- **Teensy Code**: C++ with Arduino framework and PlatformIO
- **Development**: VS Code with Remote-SSH and PlatformIO extension

## Project Structure
```
km-box/
├── pi_code/                 # Rust project for Raspberry Pi
├── teensy_code/             # PlatformIO project for Teensy 4.0
│   ├── platformio.ini       # Board configuration
│   ├── src/main.cpp         # Arduino C++ code
│   └── .vscode/             # VS Code settings
└── docs/                    # Documentation
    ├── project_log.md       # Development log
    └── copilot_instructions.md # AI assistant context
```

## Setup Instructions

### Prerequisites
- VS Code with PlatformIO and Remote-SSH extensions
- Raspberry Pi 5 with Raspbian OS
- Teensy 4.0 with Teensyduino
- SSH access to Pi configured

### Phase 1: Environment Setup
1. **SSH Key Setup**: Generate RSA key and copy to Pi for passwordless access
2. **Rust Installation**: Install Rust toolchain on Raspberry Pi
3. **PlatformIO**: Verify Teensy project builds and uploads
4. **Basic Testing**: Run "hello world" on both platforms

### Development Phases
1. **Phase 1**: Environment validation and basic connectivity ✅
2. **Phase 2**: UART communication between Pi and Teensy
3. **Phase 3**: Input capture from USB devices on Pi
4. **Phase 4**: USB HID output implementation on Teensy
5. **Phase 5**: Complete pass-through functionality
6. **Phase 6**: Advanced features and optimization

## Current Status: Phase 1 - Environment Setup
- [x] Project structure created
- [x] SSH key generated for Pi access
- [x] Teensy PlatformIO project configured
- [x] Basic "hello world" code prepared
- [ ] SSH key copied to Pi (waiting for password entry)
- [ ] Rust installed on Pi
- [ ] Initial testing completed

## Connection Details
- **Pi SSH**: otis@192.168.1.117 (key-based auth)
- **Teensy USB**: Connected for programming and serial monitor
- **UART**: Pi GPIO → Teensy Serial1 (future phases)

## Commands Quick Reference

### Build Teensy Code
```bash
# In teensy_code folder
pio run
pio run --target upload
```

### SSH to Pi
```bash
ssh pi5  # After SSH config setup
```

### Sync Code to Pi
```bash
rsync -avz --delete pi_code/ pi5:~/km_box_project/pi_code/
```

## Notes
- Monitor speed: 115200 baud for all serial communications
- LED_BUILTIN used for visual feedback during testing
- Error handling and logging implemented for debugging

---
*Project Log: See [docs/project_log.md](docs/project_log.md) for detailed development notes*
