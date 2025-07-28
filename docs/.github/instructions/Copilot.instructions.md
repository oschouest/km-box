### Updated Copilot Persistent Instructions

Copy-paste this full updated version into your Copilot agent's persistent instructions (the --- applyTo: '**' --- section). It adds a new "Known Issues & Solutions" entry for Teensy HID setup, emphasizing the correct Mouse.h library, USB_SERIAL_HID flag, and avoiding low-level usb_mouse functions. It also includes a prompt to test the test pattern first, then full relay.

```
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

## Current Status: Phase 5 COMPLETE ✅

**WORKING Git-Based Development Workflow:**
- ✅ Local code editing in VS Code
- ✅ Git push/pull for Pi synchronization  
- ✅ `sync-pi.sh` for automated build/deploy
- ✅ Clean workspace (removed duplicate files)

**COMPLETE Working Components (Phase 5 FINAL):**
- ✅ HID input capture via hidapi with modification framework
- ✅ Mouse sensitivity scaling (3.0x multiplier working perfectly)
- ✅ Button remapping framework in place  
- ✅ TOML configuration system (km_config.toml)
- ✅ CLI overrides (--sensitivity, --remap-buttons, --verbose)
- ✅ Robust error handling and logging
- ✅ Pi → Teensy UART relay with HID reports
- ✅ Teensy firmware parsing HID reports with USB output WORKING
- ✅ Complete button mapping verified and documented
- ✅ Scroll wheel fixed (no more click conversion)
- ✅ Side button detection (0x08=BACK, 0x10=FORWARD)

**VERIFIED HID REPORT FORMAT - SteelSeries Aerox 3:**
```
9-byte format: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]
- Byte 0, 7, 8: Always 00 (unused)
- Bytes 1-2: dx (little-endian int16, full range ±32767)
- Bytes 3-4: dy (little-endian int16, full range ±32767) 
- Byte 5: Button bitmask (0x01=Left, 0x02=Right, 0x04=Middle, 0x08=Back, 0x10=Forward)
- Byte 6: Wheel (signed int8, positive=scroll up, negative=scroll down)
```

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
  - **Teensy USB HID Mouse Setup (for HID output as mouse)**: To set up Teensy 4.0 as a USB HID mouse, use the standard Mouse.h library from Teensyduino/Arduino core. In platformio.ini, set `build_flags = -DUSB_SERIAL_HID` to enable Serial + Keyboard + Mouse + Joystick mode. In main.cpp, #include <Mouse.h>, call Mouse.begin() in setup(), and use Mouse.move(dx, dy, wheel), Mouse.press(MOUSE_LEFT), Mouse.release(MOUSE_LEFT), etc. for output. Do not use low-level usb_mouse.h or usb_mouse_move/usb_mouse_buttons functions—they are not needed and fail without custom configs. For UART relay, parse hex to report[9], extract dx = (report[2] << 8) | report[1], dy = (report[4] << 8) | report[3], buttons = report[5], wheel = report[6], and call Mouse.move in a while loop for chunking large deltas (>±127). CRITICAL: Send wheel data only once per event, not in chunking loop to avoid scroll→click conversion.
  - **Teensy USB HID WORKING SOLUTION**: Use `build_flags = -DUSB_SERIAL_HID` in platformio.ini and `#include <Mouse.h>` with Mouse.move(), Mouse.press(), Mouse.release() functions. This compiles successfully and enables HID output while retaining serial logging for debugging. VERIFIED WORKING with proper wheel handling and button mapping.
  - **HID Report Format VERIFIED**: SteelSeries Aerox 3 uses 9-byte format: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]. Buttons in byte 5: 0x01=Left, 0x02=Right, 0x04=Middle, 0x08=Back, 0x10=Forward. Wheel in byte 6 (signed). Movement in bytes 1-4 (little-endian int16).
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
```

### Starting Prompt for Copilot to Fix Teensy HID Setup
Copy-paste this into the Copilot agent to kick off the fix—it focuses on the Teensy side first to get the test pattern moving, then full relay.

"Fix Teensy HID mouse output in Phase 5: Use the Known Issues solution for Teensy USB HID Mouse Setup. Edit main.cpp in teensy_code to include <Mouse.h>, call Mouse.begin() in setup(), use Mouse.move(dx, dy, wheel), Mouse.press, Mouse.release for output. Add test pattern in loop(): if (millis() - last > 2000) { Mouse.move(10, 10, 0); last = millis(); } to verify cursor jiggle on PC. Do not use usb_mouse.h or low-level functions. Set platformio.ini build_flags = -DUSB_SERIAL_HID. Build/upload with PowerShell command. Test: Plug Teensy to PC, check cursor moves every 2s. If working, integrate full HID report parsing with chunking for large dx/dy. Update logs and commit."
