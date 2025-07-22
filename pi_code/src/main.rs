use std::io::{self, Read, Write};
use std::time::Duration;
use std::thread;

fn main() -> io::Result<()> {
    println!("KM-Box Pi UART Communication Test");
    println!("Connecting to Teensy via /dev/serial0...");

    // Open serial port using builder pattern
    let mut port = serialport::new("/dev/serial0", 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| {
            eprintln!("Failed to open /dev/serial0: {}", e);
            eprintln!("Make sure UART is enabled in raspi-config and user is in dialout group");
            io::Error::new(io::ErrorKind::Other, e)
        })?;

    println!("Serial port opened successfully at 115200 baud");
    println!("Hardware wiring check:");
    println!("  Pi GPIO 14 (TX) → Teensy Pin 0 (RX1)");
    println!("  Pi GPIO 15 (RX) → Teensy Pin 1 (TX1)");
    println!("  Pi GND → Teensy GND");
    println!("");

    // Commands to test
    let commands = ["ping", "test", "led_on", "led_off", "status"];
    let mut command_index = 0;

    // Clear any existing data
    let mut buffer = [0; 256];
    let _ = port.read(&mut buffer);

    println!("Starting UART communication test...");
    println!("Press Ctrl+C to stop");
    println!("");

    loop {
        let command = commands[command_index];
        let command_with_newline = format!("{}\n", command);

        // Send command
        print!("Sending: '{}' → ", command);
        io::stdout().flush()?;
        
        match port.write(command_with_newline.as_bytes()) {
            Ok(bytes_written) => {
                println!("({} bytes sent)", bytes_written);
            }
            Err(e) => {
                eprintln!("Error writing to serial port: {}", e);
                continue;
            }
        }

        // Wait a bit for response
        thread::sleep(Duration::from_millis(100));

        // Read response
        let mut response_buffer = [0; 256];
        match port.read(&mut response_buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let response = String::from_utf8_lossy(&response_buffer[..bytes_read]);
                    let response_clean = response.trim();
                    if !response_clean.is_empty() {
                        println!("Received: '{}' ({} bytes)", response_clean, bytes_read);
                    }
                } else {
                    println!("No response received");
                }
            }
            Err(e) => {
                println!("Error reading from serial port: {}", e);
            }
        }

        println!("");

        // Move to next command
        command_index = (command_index + 1) % commands.len();

        // Wait before next command
        thread::sleep(Duration::from_secs(2));
    }
}
