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
    deadzone_threshold: i16,  // Change to i16 for consistency
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
    dx: i16,  // Full int16
    dy: i16,  // Full int16
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
                dx,
                dy,
                wheel,
            })
        } else {
            None
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let dx_bytes = self.dx.to_le_bytes();
        let dy_bytes = self.dy.to_le_bytes();
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

            modified.dx = new_dx.max(-32768.0).min(32767.0) as i16;
            modified.dy = new_dy.max(-32768.0).min(32767.0) as i16;

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
            println!("Device: VID={:04x