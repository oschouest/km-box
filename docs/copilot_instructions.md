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

## Current Status: Phase 3 Complete ✅

**WORKING Git-Based Development Workflow:**
- ✅ Local code editing in VS Code
- ✅ Git push/pull for Pi synchronization  
- ✅ `sync-pi.sh` for automated build/deploy
- ✅ Clean workspace (removed duplicate files)

**Verified Working Components:**
- ✅ HID input capture via hidapi (99 mouse reports captured)
- ✅ SteelSeries mouse detection (VID=1038, PID=183a)
- ✅ Rust binaries: `km_pi` (UART relay) + `hid_test` (test only)
- ✅ UART initialization to Teensy (/dev/ttyAMA0, 9600 baud)
- ✅ GPIO communication ready (Pi GPIO 14/15 ↔ Teensy pins 0/1)

**Next Phase:** Full HID→UART→Teensy→USB HID relay testing

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

## Key Libraries/Crates - VERIFIED WORKING
- **Rust**: hidapi (HID input capture), serialport (UART), clap (CLI), log/env_logger
- **C++**: Arduino core, Keyboard.h, Mouse.h, HardwareSerial

## Development Phases - CURRENT: Phase 3 ✅
1. **Phase 1**: Environment setup and basic "hello world" validation ✅
2. **Phase 2**: UART communication between Pi and Teensy ✅  
3. **Phase 3**: Input capture on Pi (hidapi) ✅ **← COMPLETED**
4. **Phase 4**: USB HID output on Teensy ⏳ **← NEXT**
5. **Phase 5**: Complete pass-through system
6. **Phase 6**: Injection capabilities and optimization

## Coding Guidelines - VERIFIED PATTERNS
- **Rust**: Use hidapi for HID, serialport builder API, proper error handling with Result<>
- **C++**: Arduino-style setup()/loop(), avoid blocking operations
- **Communication**: Hex-encoded HID reports via UART ("HID:00ff00..." format)
- **Testing**: `./sync-pi.sh` for build/deploy, `hid_test` for validation


## Copilot Instructions
- **Shell Syntax**: Use only valid Windows PowerShell syntax. Chain commands with ';'. Use Out-File, Set-Content, Add-Content for file creation/editing (e.g., $content = "multi\nline\ntext"; $content | Out-File -FilePath file.txt -Encoding utf8).
- **No Bash**: Do not use bash heredocs (<< 'EOF'), '&&', '|', or Linux redirects (> file). Use PowerShell equivalents (e.g., New-Item, Remove-Item -Recurse -Force).
- **SSH/SCP**: Always use "pi5" alias for SSH/SCP (e.g., ssh pi5 "command", scp local_file pi5:~/remote_path). For multi-line files on Pi, use Set-Content via SSH or scp from local.
- **File Sync to Pi**: For syncing pi_code to Pi, use scp -r pi_code/ pi5:~/km_box_project/km_pi/
- **Error Handling**: If a command fails, suggest the fixed PowerShell version. Test commands in your response.
- **Output**: Provide exact, testable commands. For code changes, output full file contents. Always end your response with a "Current State Summary" section including: - Excerpt from last 3 sections of project_log.md - Git status (run git status) - List of root folders/files (run Get-ChildItem -Path . -Recurse -Depth 1) - Folder tree (run Get-ChildItem -Recurse | Tree or simulate tree output) - Any recent errors/snippets.
- **Process**: Implement one phase at a time. Output: 1. Code changes (paths, contents). 2. Build/deploy commands. 3. Test steps. 4. Log updates (append to project_log.md). 5. README updates. 6. Commit message. Automatically append to project_log.md after each phase.
- **If errors, suggest fixes. Keep iterations small. After phase, generate log/README based on success; revise if tests fail.
```
