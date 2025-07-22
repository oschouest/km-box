use evdev::{Device, EventType};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing direct event5 access...");
    
    // Try to open event5 directly
    match Device::open("/dev/input/event5") {
        Ok(mut device) => {
            println!("✓ Successfully opened /dev/input/event5");
            println!("Device name: {}", device.name().unwrap_or("Unknown"));
            
            // Check supported event types
            let supported_events = device.supported_events();
            println!("Supported events: {:?}", supported_events);
            
            if supported_events.contains(EventType::RELATIVE) {
                println!("✓ Device supports relative movement (mouse)");
            }
            
            if supported_events.contains(EventType::KEY) {
                println!("✓ Device supports key events (buttons)");
            }
            
            println!("📡 Monitoring events for 10 seconds... Move mouse or click!");
            
            let start_time = std::time::Instant::now();
            loop {
                if start_time.elapsed() > Duration::from_secs(10) {
                    break;
                }
                
                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            println!("Event: type={:?}, code={}, value={}", 
                                event.event_type(), event.code(), event.value());
                        }
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to open /dev/input/event5: {}", e);
            println!("Error details: {:?}", e);
        }
    }
    
    Ok(())
}
