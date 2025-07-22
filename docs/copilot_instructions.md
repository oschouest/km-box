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
- **SSH Alias**: Always use "pi5" for SSH/SCP commands (configured in ~/.ssh/config as Host pi5 with HostName 192.168.1.117, User otis, IdentityFile ~/.ssh/id_rsa)
- **VS Code Settings**: Ensure GitHub Copilot is enabled with inline suggestions, tab completion, and quick suggestions for all types. Remote-SSH platforms set to "linux" for pi5 and 192.168.1.117.

## Project Structure
```
km-box/
├── pi_code/           # Rust project for Pi (mirror for local editing; sync to Pi)
├── teensy_code/       # PlatformIO project for Teensy
└── docs/              # Documentation
```

## Key Libraries/Crates
- **Rust**: serialport (use builder pattern for port config, e.g., serialport::new(path, baud).open()), evdev, tokio (async runtime)
- **C++**: Arduino core, Keyboard.h, Mouse.h, HardwareSerial

## Development Phases
1. **Phase 1**: Environment setup and basic "hello world" validation
2. **Phase 2**: UART communication between Pi and Teensy
3. **Phase 3**: Input capture on Pi (evdev)
4. **Phase 4**: USB HID output on Teensy
5. **Phase 5**: Complete pass-through system
6. **Phase 6**: Injection capabilities and optimization

## Coding Guidelines
- **Rust**: Use async/await patterns, proper error handling with Result<>. For serialport, use builder API (no SerialPortSettings).
- **C++**: Arduino-style setup()/loop(), avoid blocking operations
- **Communication**: Simple text-based protocol initially, binary later
- **Testing**: Unit tests for Rust, serial monitor testing for Teensy

## Current Phase: Phase 2 - UART Communication
- Teensy side: Implemented command handler for "test", "ping", etc.
- Pi side: Implement Rust UART sender/reader

## Copilot Instructions
- **Shell Syntax**: Use only valid Windows PowerShell syntax. Chain commands with ';'. Use Out-File, Set-Content, Add-Content for file creation/editing (e.g., $content = "multi\nline\ntext"; $content | Out-File -FilePath file.txt -Encoding utf8).
- **No Bash**: Do not use bash heredocs (<< 'EOF'), '&&', '|', or Linux redirects (> file). Use PowerShell equivalents (e.g., New-Item, Remove-Item -Recurse -Force).
- **SSH/SCP**: Always use "pi5" alias for SSH/SCP (e.g., ssh pi5 "command", scp local_file pi5:~/remote_path). For multi-line files on Pi, use Set-Content via SSH or scp from local.
- **File Sync to Pi**: For syncing pi_code to Pi, use scp -r pi_code/ pi5:~/km_box_project/km_pi/
- **Error Handling**: If a command fails, suggest the fixed PowerShell version. Test commands in your response.
- **Output**: Provide exact, testable commands. For code changes, output full file contents.
- **Process**: Implement one phase at a time. Output: 1. Code changes (paths, contents). 2. Build/deploy commands. 3. Test steps. 4. Log updates. 5. README updates. 6. Commit message.
- **If errors, suggest fixes. Keep iterations small. After phase, generate log/README based on success; revise if tests fail.