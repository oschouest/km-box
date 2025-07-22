# Copilot Instructions for KM Box Project

## Project Overview
The KM Box is a low-latency keyboard/mouse pass-through and injection system for gaming, consisting of:
- **Raspberry Pi 5**: Running Rust code for input capture and processing
- **Teensy 4.0**: Running C++ code for USB HID output to gaming PC
- **Communication**: UART connection between Pi and Teensy

## Architecture
```
Gaming PC <-- USB HID --> Teensy 4.0 <-- UART --> Raspberry Pi 5 <-- USB --> Input Devices
```

## Development Environment
- **Languages**: Rust (Pi), C++ Arduino/Teensyduino (Teensy)
- **Tools**: VS Code with Remote-SSH, PlatformIO, Cargo
- **Communication**: SSH to Pi (otis@192.168.1.117), USB to Teensy

## Project Structure
```
km-box/
├── pi_code/           # Rust project for Pi
├── teensy_code/       # PlatformIO project for Teensy
└── docs/             # Documentation
```

## Key Libraries/Crates
- **Rust**: serialport, evdev, tokio (async runtime)
- **C++**: Arduino core, Keyboard.h, Mouse.h, HardwareSerial

## Development Phases
1. **Phase 1**: Environment setup and basic "hello world" validation
2. **Phase 2**: UART communication between Pi and Teensy
3. **Phase 3**: Input capture on Pi (evdev)
4. **Phase 4**: USB HID output on Teensy
5. **Phase 5**: Complete pass-through system
6. **Phase 6**: Injection capabilities and optimization

## Coding Guidelines
- **Rust**: Use async/await patterns, proper error handling with Result<>
- **C++**: Arduino-style setup()/loop(), avoid blocking operations
- **Communication**: Simple text-based protocol initially, binary later
- **Testing**: Unit tests for Rust, serial monitor testing for Teensy

## Current Phase: Phase 1 - Environment Validation
- SSH key setup for Pi access
- Rust installation and "hello world" on Pi
- PlatformIO build and upload to Teensy
- Basic task configuration in VS Code
