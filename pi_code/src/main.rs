use std::io::Write;
use std::time::Duration;
use std::fs;
use hidapi::HidApi;
use serialport::SerialPort;
use clap::Parser;
use log::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "km_pi")]
#[command(about = "KM-Box Phase 5: Input Modification Framework")]
struct Args {
    /// Target mouse vendor ID (hex)
    #[arg(long, default_value = "0x1038")]
    mouse_vid: String,

    /// Target mouse product ID (hex)  
    #[arg(long, default_value = "0x183a")]
    mouse_pid: String,

    /// Mouse sensitivity multiplier
    #[arg(long, default_value_t = 1.0)]
    sensitivity: f32,

    /// Enable button remapping (swap left/right)
    #[arg(long)]
    remap_buttons: bool,

    /// Config file path (TOML format)
    #[arg(long, default_value = "km_config.toml")]
    config: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// List all available HID devices and exit
    #[arg(long)]
    list_devices: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    sensitivity: f32,
    remap_buttons: bool,
    deadzone_threshold: i8,
    max_acceleration: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            remap_buttons: false,
            deadzone_threshold: 1,
            max_acceleration: 10.0,
        }
    }
}

#[derive(Debug, Clone)]
struct MouseReport {
    buttons: u8,
    dx: i8,
    dy: i8,
    wheel: i8,
}

impl MouseReport {
    fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() >= 9 {
            // Parse 9-byte HID report: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]
            let dx = i16::from_le_bytes([data[1], data[2]]);
            let dy = i16::from_le_bytes([data[3], data[4]]);
            let buttons = data[5];
            let wheel = data[6] as i8;
            
            Some(MouseReport {
                buttons,
                dx: dx as i8,  // Still store as i8 for internal use
                dy: dy as i8,
                wheel,
            })
        } else if data.len() >= 4 {
            // Fallback to 4-byte format
            Some(MouseReport {
                buttons: data[0],
                dx: data[1] as i8,
                dy: data[2] as i8,
                wheel: data[3] as i8,
            })
        } else {
            None
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Send 9-byte format preserving full int16 range
        let dx_bytes = (self.dx as i16).to_le_bytes();
        let dy_bytes = (self.dy as i16).to_le_bytes();
        vec![0x00, dx_bytes[0], dx_bytes[1], dy_bytes[0], dy_bytes[1], self.buttons, self.wheel as u8, 0x00, 0x00]
    }
}

struct InputModifier {
    config: Config,
}

impl InputModifier {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn modify_mouse_report(&self, report: MouseReport) -> Result<MouseReport, String> {
        let mut modified = report.clone();

        // Apply sensitivity scaling with overflow protection
        if self.config.sensitivity != 1.0 {
            let new_dx = (modified.dx as f32 * self.config.sensitivity).round();
            let new_dy = (modified.dy as f32 * self.config.sensitivity).round();

            // Clamp to i8 range and apply acceleration limiting
            modified.dx = new_dx.max(-127.0).min(127.0) as i8;
            modified.dy = new_dy.max(-127.0).min(127.0) as i8;

            // Apply deadzone
            if modified.dx.abs() < self.config.deadzone_threshold {
                modified.dx = 0;
            }
            if modified.dy.abs() < self.config.deadzone_threshold {
                modified.dy = 0;
            }

            debug!("Sensitivity: {:.2} | Original: ({}, {}) -> Modified: ({}, {})", 
                   self.config.sensitivity, report.dx, report.dy, modified.dx, modified.dy);
        }

        // Apply button remapping (swap left/right mouse buttons)
        if self.config.remap_buttons {
            let left_pressed = (modified.buttons & 0x01) != 0;
            let right_pressed = (modified.buttons & 0x02) != 0;

            // Clear left and right button bits
            modified.buttons &= !0x03;

            // Swap them
            if left_pressed {
                modified.buttons |= 0x02; // Set right button
            }
            if right_pressed {
                modified.buttons |= 0x01; // Set left button
            }

            if left_pressed || right_pressed {
                debug!("Button remap: Original buttons={:02x} -> Modified buttons={:02x}", 
                       report.buttons, modified.buttons);
            }
        }

        Ok(modified)
    }
}

fn load_config(config_path: &str, args: &Args) -> Config {
    let mut config = if let Ok(content) = fs::read_to_string(config_path) {
        match toml::from_str(&content) {
            Ok(cfg) => {
                info!("✓ Loaded configuration from {}", config_path);
                cfg
            }
            Err(e) => {
                warn!("Failed to parse config file {}: {}. Using defaults.", config_path, e);
                Config::default()
            }
        }
    } else {
        info!("Config file {} not found. Using defaults.", config_path);
        Config::default()
    };

    // Override with CLI arguments
    config.sensitivity = args.sensitivity;
    config.remap_buttons = args.remap_buttons;

    // Save current config back to file for future reference
    if let Ok(toml_content) = toml::to_string_pretty(&config) {
        if let Err(e) = fs::write(config_path, toml_content) {
            warn!("Failed to save config to {}: {}", config_path, e);
        } else {
            debug!("✓ Saved current config to {}", config_path);
        }
    }

    config
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Setup logging
    if args.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
    
    info!("=== KM-Box Phase 5: Input Modification Framework ===");
    info!("Enhanced input processing with sensitivity scaling and button remapping");

    // List devices if requested
    if args.list_devices {
        info!("Enumerating all HID devices...");
        let api = HidApi::new()?;
        for device_info in api.device_list() {
            println!("Device: VID={:04x} PID={:04x} Manufacturer={:?} Product={:?}", 
                device_info.vendor_id(), 
                device_info.product_id(),
                device_info.manufacturer_string(),
                device_info.product_string());
        }
        return Ok(());
    }

    // Parse target VIDs/PIDs
    let mouse_vid = u16::from_str_radix(&args.mouse_vid.trim_start_matches("0x"), 16)?;
    let mouse_pid = u16::from_str_radix(&args.mouse_pid.trim_start_matches("0x"), 16)?;
    
    info!("Target mouse: VID={:04x} PID={:04x}", mouse_vid, mouse_pid);

    // Load configuration
    let config = load_config(&args.config, &args);
    info!("Configuration: sensitivity={:.2}, remap_buttons={}, deadzone={}", 
          config.sensitivity, config.remap_buttons, config.deadzone_threshold);

    // Initialize input modifier
    let modifier = InputModifier::new(config);

    // Open HID device
    let api = HidApi::new()?;
    let device = api.open(mouse_vid, mouse_pid)
        .map_err(|e| {
            error!("Failed to open mouse device VID={:04x} PID={:04x}: {}", mouse_vid, mouse_pid, e);
            error!("Make sure the device is connected and you have proper permissions");
            e
        })?;
    
    info!("✓ Connected to mouse device");

    // Open UART connection to Teensy
    let mut uart_port = serialport::new("/dev/ttyAMA0", 115200)
        .timeout(Duration::from_millis(100))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .open()
        .map_err(|e| {
            error!("Failed to open UART /dev/ttyAMA0: {}", e);
            error!("Hardware check: Pi GPIO 14/15 ↔ Teensy pins 0/1, GND connected");
            e
        })?;

    info!("✓ UART connected to Teensy at 115200 baud");

    // Send initialization signal
    let init_msg = "phase5_start\n";
    uart_port.write_all(init_msg.as_bytes())?;
    uart_port.flush()?;
    info!("✓ Sent Phase 5 initialization signal to Teensy");

    info!("Starting input modification loop... Press Ctrl+C to stop");

    let mut buf = [0u8; 64];
    let mut report_count = 0u64;
    let mut modified_count = 0u64;

    loop {
        match device.read_timeout(&mut buf, 100) {
            Ok(size) if size > 0 => {
                report_count += 1;

                if size >= 9 {
                    // Extract full int16 values from raw buffer
                    let buttons = buf[5];
                    let dx = i16::from_le_bytes([buf[1], buf[2]]);
                    let dy = i16::from_le_bytes([buf[3], buf[4]]);
                    let wheel = buf[6] as i8;

                    debug!("Raw HID report: buttons={:02x}, dx={}, dy={}, wheel={}", buttons, dx, dy, wheel);

                    // Apply sensitivity scaling to full int16 values
                    let scaled_dx = (dx as f32 * modifier.config.sensitivity).round() as i16;
                    let scaled_dy = (dy as f32 * modifier.config.sensitivity).round() as i16;

                    // Apply button remapping if enabled
                    let final_buttons = if modifier.config.remap_buttons {
                        let mut remapped = buttons;
                        if buttons & 0x01 != 0 { remapped |= 0x02; remapped &= !0x01; } // Left -> Right
                        if buttons & 0x02 != 0 { remapped |= 0x01; remapped &= !0x02; } // Right -> Left
                        remapped
                    } else {
                        buttons
                    };

                    // Check if modifications were applied
                    let was_modified = dx != scaled_dx || dy != scaled_dy || buttons != final_buttons;
                    if was_modified {
                        modified_count += 1;
                        debug!("Modified report: buttons={:02x}, dx={}, dy={}, wheel={}", final_buttons, scaled_dx, scaled_dy, wheel);
                    }

                    // Repack into full 9-byte buffer preserving int16
                    let mut modified_buf = buf.clone();
                    modified_buf[1..3].copy_from_slice(&scaled_dx.to_le_bytes());
                    modified_buf[3..5].copy_from_slice(&scaled_dy.to_le_bytes());
                    modified_buf[5] = final_buttons;
                    modified_buf[6] = wheel as u8;

                    // Encode full 9-byte buffer to hex (18 chars) and send
                    let hex_data = hex::encode(&modified_buf[..size]);
                    let uart_msg = format!("HID:{}\n", hex_data);
                    
                    if let Err(e) = uart_port.write_all(uart_msg.as_bytes()) {
                        error!("UART write failed: {}", e);
                    } else if let Err(e) = uart_port.flush() {
                        error!("UART flush failed: {}", e);
                    }
                } else {
                    // Fallback for smaller reports - send as-is
                    warn!("Unexpected report size: {} bytes, expected 9", size);
                    let hex_data = hex::encode(&buf[..size]);
                    let uart_msg = format!("HID:{}\n", hex_data);
                    if let Err(e) = uart_port.write_all(uart_msg.as_bytes()) {
                        error!("UART write failed: {}", e);
                    }
                }

                // Log statistics every 1000 reports
                if report_count % 1000 == 0 {
                    info!("Stats: {} reports processed, {} modified ({:.1}%)", 
                          report_count, modified_count, 
                          (modified_count as f64 / report_count as f64) * 100.0);
                }
            }
            Ok(_) => {
                // No data available, continue
                continue;
            }
            Err(e) => {
                warn!("HID read error: {}", e);
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}
