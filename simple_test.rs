use hidapi::HidApi;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing HID input capture...");
    
    let api = HidApi::new()?;
    let device = api.open(0x1038, 0x183a)?;
    
    println!("✓ Opened SteelSeries device");
    
    let mut buf = [0u8; 64];
    
    for i in 0..10 {
        match device.read_timeout(&mut buf, 1000) {
            Ok(size) if size > 0 => {
                println!("Report {}: {:?}", i, &buf[..size]);
            }
            Ok(_) => {
                println!("Timeout {}", i);
            }
            Err(e) => {
                println!("Error {}: {}", i, e);
            }
        }
    }
    
    Ok(())
}
