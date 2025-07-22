use std::time::Duration;
use hidapi::{HidApi, HidDevice};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "km_pi_test")]
#[command(about = "KM-Box HID Test - No UART")]
struct Args {
    /// Target mouse vendor ID (hex)
    #[arg(long, default_value = "0x1038")]
    mouse_vid: String,

    /// Target mouse product ID (hex)  
    #[arg(long, default_value = "0x183a")]
    mouse_pid: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("=== KM-Box HID Test ===");
    println!("Testing HID input capture without UART...");

    // Parse target VIDs/PIDs
    let mouse_vid = u16::from_str_radix(&args.mouse_vid.trim_start_matches("0x"), 16)?;
    let mouse_pid = u16::from_str_radix(&args.mouse_pid.trim_start_matches("0x"), 16)?;
    
    println!("Target mouse: VID={:04x} PID={:04x}", mouse_vid, mouse_pid);

    // Initialize HID API
    let api = HidApi::new()?;

    // Open mouse device
    let device = api.open(mouse_vid, mouse_pid)
        .map_err(|e| {
            eprintln!("Failed to open mouse device VID={:04x} PID={:04x}: {}", mouse_vid, mouse_pid, e);
            e
        })?;
    
    println!("✓ Opened mouse device VID={:04x} PID={:04x}", mouse_vid, mouse_pid);
    println!("Monitoring mouse for 10 seconds... Move your mouse!");

    let mut buf = [0u8; 64];
    let mut count = 0;
    
    for i in 0..100 {  // 10 seconds at 100ms intervals
        match device.read_timeout(&mut buf, 100) {
            Ok(size) if size > 0 => {
                count += 1;
                println!("Report {}: {:02x?}", count, &buf[..size]);
            }
            Ok(_) => {
                // Timeout, print a dot to show we are alive
                if i % 10 == 0 {
                    print!(".");
                    std::io::Write::flush(&mut std::io::stdout())?;
                }
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
            }
        }
    }
    
    println!("");
    println!("Test complete! Captured {} reports", count);
    Ok(())
}
