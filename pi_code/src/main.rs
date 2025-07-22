use std::io::Write;
use std::time::Duration;
use hidapi::{HidApi, HidDevice, HidResult};
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
    #[arg(long, default_value = "0x183a")]
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
    Mouse(Vec<u8>),
    Keyboard(Vec<u8>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Setup logging
    if args.verbose {
        env_logger::init();
    }
    
    info!("=== KM-Box Phase 3: HID Input Capture & UART Relay ===");
    info!("Initializing HID input capture and UART communication...");

    // Initialize HID API
    let api = HidApi::new()?;

    // List devices if requested
    if args.list_devices {
        info!("Enumerating all HID devices...");
        for device in api.device_list() {
            println!("Device: VID={:04x} PID={:04x} Path={:?}", 
                device.vendor_id(), device.product_id(), device.path().to_string_lossy());
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
    let init_msg = "phase3_hidapi_start\n";
    let mut uart_clone = uart_port.try_clone()?;
    uart_clone.write_all(init_msg.as_bytes())?;
    uart_clone.flush()?;
    info!("✓ Sent initialization signal to Teensy");

    // Open mouse device
    let mouse_device = api.open(mouse_vid, mouse_pid)
        .map_err(|e| {
            error!("Failed to open mouse device VID={:04x} PID={:04x}: {}", mouse_vid, mouse_pid, e);
            e
        })?;
    
    info!("✓ Opened mouse device VID={:04x} PID={:04x}", mouse_vid, mouse_pid);

    // Start mouse monitoring task
    let mouse_tx = tx.clone();
    let mouse_task = tokio::task::spawn_blocking(move || {
        monitor_mouse_device(mouse_device, mouse_tx)
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
    tokio::try_join!(mouse_task, uart_task)?;

    Ok(())
}

fn monitor_mouse_device(device: HidDevice, tx: mpsc::Sender<InputEvent>) -> Result<(), String> {
    info!("🖱️ Starting mouse monitoring...");
    
    let mut buf = [0u8; 64];
    
    loop {
        match device.read_timeout(&mut buf, 100) {
            Ok(size) if size > 0 => {
                debug!("Mouse HID report: {:?}", &buf[..size]);
                let data = buf[..size].to_vec();
                
                // Send to UART relay task
                if let Err(e) = tx.blocking_send(InputEvent::Mouse(data)) {
                    error!("Failed to send mouse event: {}", e);
                    break;
                }
            }
            Ok(_) => {
                // Timeout, continue
            }
            Err(e) => {
                warn!("Mouse read error: {}", e);
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
    
    Ok(())
}

async fn send_to_teensy(uart: &mut Box<dyn SerialPort>, event: InputEvent) -> Result<(), Box<dyn std::error::Error>> {
    let (prefix, data) = match event {
        InputEvent::Mouse(data) => ("MOUSE:", data),
        InputEvent::Keyboard(data) => ("KEYBOARD:", data),
    };
    
    let hex_data = hex::encode(&data);
    let message = format!("{}{}", prefix, hex_data);
    
    debug!("Sending to Teensy: {}", message);
    
    uart.write_all(message.as_bytes())?;
    uart.write_all(b"\n")?;
    uart.flush()?;
    
    Ok(())
}
