---
applyTo: '**'
---
# Copilot Instructions for KM Box Project

## Project Overview
The KM Box is an AI-powered gaming enhancement system, evolving from a simple input relay to a complete adaptive gaming intelligence platform:

**Phase 1-3 (Current)**: Basic hardware relay (Pi↔Teensy)
**Phase 4-6**: Input modification & recoil compensation 
**Phase 7-9**: AI integration with Aimmy GitHub project
**Phase 10-12**: Adaptive OCR-based recoil intelligence

### Ultimate Vision:
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

**Target Games**: Rainbow Six Siege (priority), CS2, Valorant, Apex
**AI Integration**: Aimmy project for visual analysis & decision making
**Adaptive Recoil**: OCR weapon detection, auto-compensation

## Architecture
```
Gaming PC <-- USB HID --> Teensy 4.0 <-- UART --> Raspberry Pi 5 <-- USB --> Input Devices
```

## Current Status: Phase 5 Complete ✅

**WORKING Git-Based Development Workflow:**
- ✅ Local code editing in VS Code
- ✅ Git push/pull for Pi synchronization  
- ✅ `sync-pi.sh` for automated build/deploy
- ✅ Clean workspace (removed duplicate files)

**Verified Working Components (Phase 5):**
- ✅ HID input capture via hidapi with modification framework
- ✅ Mouse sensitivity scaling (1.5x multiplier working)
- ✅ Button remapping framework in place
- ✅ TOML configuration system (km_config.toml)
- ✅ CLI overrides (--sensitivity, --remap-buttons, --verbose)
- ✅ Robust error handling and logging
- ✅ Pi → Teensy UART relay with HID reports
- ✅ Teensy firmware parsing HID reports (USB output pending)

**Next Phase:** Phase 6 Recoil Compensation Engine, then Aimmy AI integration

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

## Development Phases - CURRENT: Phase 5 ✅
1. **Phase 1**: Environment setup and basic "hello world" validation ✅
2. **Phase 2**: UART communication between Pi and Teensy ✅  
3. **Phase 3**: Input capture on Pi (hidapi) ✅
4. **Phase 4**: USB HID output on Teensy ✅
5. **Phase 5**: Input modification framework ✅ **← COMPLETED**
6. **Phase 6**: Recoil compensation (R6S priority) ⏳ **← NEXT**
7. **Phase 7**: Network API for external control
8. **Phase 8**: Aimmy AI integration foundation
9. **Phase 9**: OCR-based adaptive recoil intelligence  
10. **Phase 10**: Full AI enhancement system
11. **Phase 11**: Production optimization (<1ms latency)
12. **Phase 12**: Ecosystem & community tools

## Coding Guidelines - VERIFIED PATTERNS
- **Rust**: Use hidapi for HID, serialport builder API, proper error handling with Result<>
- **C++**: Arduino-style setup()/loop(), avoid blocking operations
- **Communication**: Hex-encoded HID reports via UART ("HID:00ff00..." format)
- **Testing**: `./sync-pi.sh` for build/deploy, `hid_test` for validation


## Copilot Instructions
- **Terminal Management**: ALWAYS close unnecessary terminals after operations. Use only one terminal per task. Clean up SSH sessions and background processes immediately after use. Do not leave multiple terminals running.
- **Known Issues & Solutions**: ALWAYS check this section before attempting fixes to avoid repeating failed solutions:
  - **Teensy USB HID WORKING SOLUTION**: Use `build_flags = -DUSB_SERIAL_HID` in platformio.ini and `#include <Mouse.h>` with Mouse.move(), Mouse.press(), Mouse.release() functions. This compiles successfully and enables HID output while retaining serial logging for debugging. Verify by adding test code like `Mouse.move(5, 5); delay(500);` in loop() and checking cursor movement on the connected PC.
  - **Teensy USB HID Libraries FAILED**: Mouse.h, usb_mouse.h, usb_mouse_move(), usb_mouse_buttons() are NOT available without proper build flags. Do not attempt to use low-level usb_mouse functions without USB_SERIAL_HID flag.
  - **PlatformIO Upload Path**: Only use full absolute path: `Set-Location "c:\Users\oscho\OneDrive - OTS Consulting Management Customs\vs-code-workspace\km-box\teensy_code"` - relative paths fail
  - **PowerShell Syntax**: Use `&` operator, not `&&` or `;` for command chaining in PowerShell
  - **Serial Monitor Access**: Direct PlatformIO serial monitor often fails with permission errors - use SSH to Pi for monitoring instead
  - **Pi Cargo Path**: Use full path `~/.cargo/bin/cargo` and specify binary `--bin km_pi` for execution
- **PlatformIO Commands on Windows**: CRITICAL - Always use the correct PowerShell syntax with call operator (&). The ONLY working command is: `Set-Location "c:\Users\oscho\OneDrive - OTS Consulting Management Customs\vs-code-workspace\km-box\teensy_code"; & "C:\Users\oscho\.platformio\penv\Scripts\platformio.exe" run --target upload`. Use full absolute paths. DO NOT use semicolons (;), DO NOT use && operators, DO NOT use cd command. This has been tested and works. Any other syntax will fail.
- **Shell Syntax**: Use only valid Windows PowerShell syntax. Chain commands with ';'. Use Out-File, Set-Content, Add-Content for file creation/editing (e.g., $content = "multi\nline\ntext"; $content | Out-File -FilePath file.txt -Encoding utf8).
- **No Bash**: Do not use bash heredocs (<< 'EOF'), '&&', '|', or Linux redirects (> file). Use PowerShell equivalents (e.g., New-Item, Remove-Item -Recurse -Force).
- **SSH/SCP**: Always use "pi5" alias for SSH/SCP (e.g., ssh pi5 "command", scp local_file pi5:~/remote_path). For multi-line files on Pi, use Set-Content via SSH or scp from local.
- **Error Handling**: If a command fails, suggest the fixed PowerShell version. Test commands in your response.
- **Output**: Provide exact, testable commands. For code changes, output full file contents. Always end your response with a "Current State Summary" section including: - Excerpt from last 3 sections of project_log.md - Git status (run git status) - List of root folders/files (run Get-ChildItem -Path . -Recurse -Depth 1) - Folder tree (run Get-ChildItem -Recurse | Tree or simulate tree output) - Any recent errors/snippets.
- **Process**: Implement one phase at a time. Output: 1. Code changes (paths, contents). 2. Build/deploy commands. 3. Test steps. 4. Log updates (append to project_log.md). 5. README updates. 6. Commit message. Automatically append to project_log.md after each phase.
- **If errors, suggest fixes. Keep iterations small. After phase, generate log/README based on success; revise if tests fail.


