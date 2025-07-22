use std::io::{self, Write};
use std::time::Duration;
use evdev::{Device, InputEventKind};
use serialport::SerialPort;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KM-Box Phase 3: Input Capture & UART Relay ===");
    println!("Initializing evdev input capture and UART communication...\n");

    // Open UART connection to Teensy
    let mut uart_port = serialport::new("/dev/ttyAMA0", 9600)
        .timeout(Duration::from_millis(100))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .open()
        .map_err(|e| {
            eprintln!("Failed to open UART /dev/ttyAMA0: {}", e);
            eprintln!("Hardware check: Pi GPIO 14/15 ↔ Teensy pins 0/1, GND connected");
            e
        })?;

    println!("✓ UART connected to Teensy at 9600 baud");

    // Send initialization ping to Teensy
    let init_msg = "phase3_start\n";
    uart_port.write_all(init_msg.as_bytes())?;
    uart_port.flush()?;
    println!("✓ Sent initialization signal to Teensy");

    // Discover input devices
    let input_devices = discover_input_devices().await?;
    if input_devices.is_empty() {
        eprintln!("No suitable input devices found in /dev/input/");
        eprintln!("Make sure keyboard/mouse are connected to Pi");
        return Ok(());
    }

    println!("✓ Found {} input device(s)", input_devices.len());
    for (path, name) in &input_devices {
        println!("  - {}: {}", path, name);
    }

    // Start input capture loop
    println!("\n🎯 Starting input capture loop...");
    println!("Press keys or move mouse - events will be sent to Teensy");
    println!("Press Ctrl+C to stop\n");

    // Open first available input device for testing
    if let Some((device_path, _)) = input_devices.first() {
        let mut device = Device::open(device_path)?;
        println!("📡 Monitoring: {}", device_path);

        loop {
            // Non-blocking event read with timeout
            let events = device.fetch_events();
            match events {
                Ok(events) => {
                    for event in events {
                        if let InputEventKind::Key(key) = event.kind() {
                            let message = format!("key:{:?}:{}\n", key, event.value());
                            send_to_teensy(&mut uart_port, &message).await?;
                        } else if let InputEventKind::RelAxis(axis) = event.kind() {
                            let message = format!("mouse:{:?}:{}\n", axis, event.value());
                            send_to_teensy(&mut uart_port, &message).await?;
                        }
                    }
                }
                Err(_) => {
                    // No events available, small delay to prevent busy loop
                    sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }

    Ok(())
}

async fn discover_input_devices() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut devices = Vec::new();
    
    // Scan /dev/input/event* devices
    for entry in std::fs::read_dir("/dev/input/")? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                if filename_str.starts_with("event") {
                    let path_str = path.to_string_lossy().to_string();
                    
                    // Try to open device to get its name
                    match Device::open(&path_str) {
                        Ok(device) => {
                            let name = device.name().unwrap_or("Unknown Device").to_string();
                            
                            // Filter for keyboards and mice
                            let name_lower = name.to_lowercase();
                            if name_lower.contains("keyboard") || 
                               name_lower.contains("mouse") || 
                               name_lower.contains("trackpad") ||
                               name_lower.contains("touchpad") {
                                devices.push((path_str, name));
                            }
                        }
                        Err(_) => {
                            // Device might be busy or require elevated permissions
                            continue;
                        }
                    }
                }
            }
        }
    }
    
    Ok(devices)
}

async fn send_to_teensy(port: &mut Box<dyn SerialPort>, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    print!("📤 {}", message.trim());
    std::io::stdout().flush()?;
    
    port.write_all(message.as_bytes())?;
    port.flush()?;
    
    // Brief delay to prevent overwhelming the UART
    sleep(Duration::from_millis(5)).await;
    
    Ok(())
}
