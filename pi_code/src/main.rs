use std::io::Write;
use std::time::Duration;
use async_hid::{HidResult, DeviceInfo, AsyncHidRead, HidBackend};
use serialport::SerialPort;
use tokio::time::sleep;
use tokio::sync::mpsc;
use clap::Parser;
use log::{info, warn, error, debug};

#[derive(Parser, Debug)]
#[command(name = "km_pi")]
#[command(about = "KM-Box Phase 3: HID Input Capture & UART Relay")]
struct Args {
    /// Target mouse vendor ID (hex)
    #[arg(long, default_value = "0x1038")]
    mouse_vid: String,

    /// Target mouse product ID (hex)  
    #[arg(long, default_value = "0x1384")]
    mouse_pid: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// List all available HID devices and exit
    #[arg(long)]
    list_devices: bool,
}

#[derive(Debug)]
enum InputEvent {
    Mouse(String),
    Keyboard(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Setup logging
    if args.verbose {
        env_logger::init();
    }
    
    info!("=== KM-Box Phase 3: Async HID Input Capture & UART Relay ===");
    info!("Initializing async-hid input capture and UART communication...");

    // List devices if requested
    if args.list_devices {
        info!("Enumerating all HID devices...");
        let backend = HidBackend::new()?;
        let devices = backend.enumerate()?;
        
        for device in devices {
            println!("Device: VID={:04x} PID={:04x} Usage={:04x} Path={:?}", 
                device.vendor_id(), device.product_id(), device.usage(), device.path());
        }
        return Ok(());
    }

    // Parse target VIDs/PIDs
    let mouse_vid = u16::from_str_radix(&args.mouse_vid.trim_start_matches("0x"), 16)?;
    let mouse_pid = u16::from_str_radix(&args.mouse_pid.trim_start_matches("0x"), 16)?;
    
    info!("Target mouse: VID={:04x} PID={:04x}", mouse_vid, mouse_pid);

    // Setup UART communication channel
    let (tx, mut rx) = mpsc::channel::<InputEvent>(100);

    // Open UART connection to Teensy
    let uart_port = serialport::new("/dev/ttyAMA0", 9600)
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

    info!("✓ UART connected to Teensy at 9600 baud");

    // Send initialization ping to Teensy
    let init_msg = "phase3_async_start\n";
    let mut uart_clone = uart_port.try_clone()?;
    uart_clone.write_all(init_msg.as_bytes())?;
    uart_clone.flush()?;
    info!("✓ Sent initialization signal to Teensy");

    // Start mouse monitoring task
    let mouse_tx = tx.clone();
    let mouse_task = tokio::spawn(async move {
        if let Err(e) = monitor_mouse_device(mouse_vid, mouse_pid, mouse_tx).await {
            error!("Mouse monitoring failed: {}", e);
        }
    });

    // Start keyboard monitoring task (using generic HID keyboard usage)
    let keyboard_tx = tx.clone();
    let keyboard_task = tokio::spawn(async move {
        if let Err(e) = monitor_keyboard_device(keyboard_tx).await {
            error!("Keyboard monitoring failed: {}", e);
        }
    });

    // UART relay task
    let uart_task = tokio::spawn(async move {
        let mut uart = uart_port;
        while let Some(event) = rx.recv().await {
            if let Err(e) = send_to_teensy(&mut uart, event).await {
                error!("UART send failed: {}", e);
            }
        }
    });

    info!("✓ All monitoring tasks started");
    info!("Monitoring for input events... Press Ctrl+C to stop");

    // Wait for all tasks (they run indefinitely unless error)
    tokio::try_join!(mouse_task, keyboard_task, uart_task)?;

    Ok(())
}

async fn monitor_mouse_device(vid: u16, pid: u16, tx: mpsc::Sender<InputEvent>) -> HidResult<()> {
    info!("🖱️ Starting mouse monitor for VID={:04x} PID={:04x}", vid, pid);
    
    let backend = HidBackend::new()?;
    let devices = backend.enumerate()?;
    
    // Find target mouse device
    let mouse_device = devices.into_iter()
        .find(|d| d.vendor_id() == vid && d.product_id() == pid)
        .ok_or_else(|| format!("Mouse device not found: VID={:04x} PID={:04x}", vid, pid))?;
    
    info!("✓ Found mouse device: {:?}", mouse_device.path());
    
    let mut device = backend.open_device(&mouse_device)?;
    
    loop {
        match device.read().await {
            Ok(data) => {
                debug!("Mouse HID report: {:?}", data);
                let hex_data = hex::encode(&data);
                let message = format!("MOUSE:{}", hex_data);
                
                if let Err(e) = tx.send(InputEvent::Mouse(message)).await {
                    error!("Failed to send mouse event: {}", e);
                    break;
                }
            }
            Err(e) => {
                warn!("Mouse read error: {}", e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
    
    Ok(())
}

async fn monitor_keyboard_device(tx: mpsc::Sender<InputEvent>) -> HidResult<()> {
    info!("⌨️ Starting keyboard monitor for generic HID keyboard");
    
    let backend = HidBackend::new()?;
    let devices = backend.enumerate()?;
    
    // Find a keyboard device (usage 0x0106 = keyboard)
    let keyboard_device = devices.into_iter()
        .find(|d| d.usage() == 0x0106)
        .ok_or_else(|| "No HID keyboard found")?;
    
    info!("✓ Found keyboard device: VID={:04x} PID={:04x} Usage={:04x}", 
          keyboard_device.vendor_id(), keyboard_device.product_id(), keyboard_device.usage());
    
    let mut device = backend.open_device(&keyboard_device)?;
    
    loop {
        match device.read().await {
            Ok(data) => {
                debug!("Keyboard HID report: {:?}", data);
                let hex_data = hex::encode(&data);
                let message = format!("KEYBOARD:{}", hex_data);
                
                if let Err(e) = tx.send(InputEvent::Keyboard(message)).await {
                    error!("Failed to send keyboard event: {}", e);
                    break;
                }
            }
            Err(e) => {
                warn!("Keyboard read error: {}", e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
    
    Ok(())
}

async fn send_to_teensy(uart: &mut Box<dyn SerialPort>, event: InputEvent) -> Result<(), Box<dyn std::error::Error>> {
    let message = match event {
        InputEvent::Mouse(data) => data,
        InputEvent::Keyboard(data) => data,
    };
    
    debug!("Sending to Teensy: {}", message);
    
    uart.write_all(message.as_bytes())?;
    uart.write_all(b"\n")?;
    uart.flush()?;
    
    Ok(())
}
