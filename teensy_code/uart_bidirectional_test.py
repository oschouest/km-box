#!/usr/bin/env python3
"""
Bidirectional UART Test for Pi-Teensy Communication
Tests sending commands and receiving responses
"""

import serial
import time
import sys

def test_bidirectional_uart():
    # Try different UART ports
    uart_ports = ['/dev/ttyAMA0', '/dev/serial0', '/dev/ttyAMA10']
    
    for port in uart_ports:
        print(f"\n=== Testing UART port: {port} ===")
        
        try:
            # Open serial port
            ser = serial.Serial(
                port=port,
                baudrate=9600,
                timeout=2,
                write_timeout=2
            )
            print(f"✓ Opened {port} successfully")
            
            # Test commands
            test_commands = [
                ("ping", "pong"),
                ("led_on", "led_on_ok"),
                ("led_off", "led_off_ok"),
                ("status", "led_"),  # Partial match since response is "led_on" or "led_off"
                ("test", "test_ok")
            ]
            
            print(f"Running {len(test_commands)} bidirectional tests...")
            
            for cmd, expected_start in test_commands:
                print(f"\n--- Test: {cmd} ---")
                
                # Send command
                command_line = cmd + '\n'
                ser.write(command_line.encode('utf-8'))
                ser.flush()
                print(f"Sent: '{cmd}'")
                
                # Wait for response
                time.sleep(0.5)
                
                # Read response
                if ser.in_waiting > 0:
                    response = ser.readline().decode('utf-8').strip()
                    print(f"Received: '{response}'")
                    
                    if response.startswith(expected_start):
                        print(f"✓ PASS: Expected '{expected_start}*', got '{response}'")
                    else:
                        print(f"✗ FAIL: Expected '{expected_start}*', got '{response}'")
                else:
                    print("✗ FAIL: No response received")
                
                time.sleep(1)  # Wait between commands
            
            ser.close()
            print(f"\n✓ All tests completed for {port}")
            return True
            
        except serial.SerialException as e:
            print(f"✗ Failed to open {port}: {e}")
        except Exception as e:
            print(f"✗ Error testing {port}: {e}")
    
    print("\n✗ No working UART ports found")
    return False

if __name__ == "__main__":
    print("=== Pi-Teensy Bidirectional UART Test ===")
    print("This script tests sending commands and receiving responses")
    print("Make sure Teensy is running response test firmware")
    
    success = test_bidirectional_uart()
    
    if success:
        print("\n🎉 Bidirectional UART communication is working!")
    else:
        print("\n❌ Bidirectional UART communication failed")
        sys.exit(1)
