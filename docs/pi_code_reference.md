# Pi Code Reference

The Rust project is located on the Pi at: `~/km_box_project/km_pi/`

## Current Status:
- ✅ Project created: `cargo new km_pi --bin`
- ✅ Compiles successfully: `cargo run`
- ✅ Outputs: "Hello, world!"

## Future Dependencies:
```toml
[dependencies]
serialport = "4.3"      # For UART communication with Teensy
evdev = "0.12"          # For reading input device events
tokio = { version = "1.0", features = ["full"] }  # Async runtime
```

## Development Workflow:
1. Edit code locally (when pi_code folder is properly set up)
2. Sync to Pi: `rsync -avz pi_code/ pi5:~/km_box_project/`
3. Build on Pi: `ssh pi5 "cd ~/km_box_project/km_pi && cargo build"`
4. Run on Pi: `ssh pi5 "cd ~/km_box_project/km_pi && cargo run"`
