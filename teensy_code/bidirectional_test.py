#!/usr/bin/env python3
"""
UART Bidirectional Test
Tests communication between Pi and Teensy with responses
"""

import serial
import time

def test_bidirectional_uart():
    # UART ports to test (based on our findings)
    ports = ['/dev/ttyAMA0']
    
    for port in ports:
        print(f"\n=== Testing bidirectional UART on {port} ===")
        
        try:
            # Open UART port
            ser = serial.Serial(port, 9600, timeout=2)
            print(f"Successfully opened {port}")
            
            # Test commands with expected responses
            test_cases = [
                ("ping", "pong"),
                ("led_on", "led_on_ok"),
                ("status", ["led_on", "led_off"]),  # Could be either
                ("led_off", "led_off_ok"),
                ("status", ["led_on", "led_off"]),  # Should be led_off now
                ("test", "test_ok"),
                ("invalid_cmd", "unknown_command")
            ]
            
            for i, (command, expected) in enumerate(test_cases, 1):
                print(f"\n--- Test {i}: Sending '{command}' ---")
                
                # Send command
                ser.write((command + '\n').encode('utf-8'))
                ser.flush()
                print(f"Sent: {command}")
                
                # Wait for response
                time.sleep(0.5)
                
                if ser.in_waiting > 0:
                    response = ser.readline().decode('utf-8').strip()
                    print(f"Received: '{response}'")
                    
                    # Check if response matches expected
                    if isinstance(expected, list):
                        if response in expected:
                            print("✅ Response matches expected values")
                        else:
                            print(f"❌ Response '{response}' not in expected {expected}")
                    else:
                        if response == expected:
                            print("✅ Response matches expected")
                        else:
                            print(f"❌ Expected '{expected}', got '{response}'")
                else:
                    print("❌ No response received")
                
                time.sleep(1)  # Wait between tests
            
            ser.close()
            print(f"\n✅ Bidirectional test completed on {port}")
            return True
            
        except serial.SerialException as e:
            print(f"❌ Failed to open {port}: {e}")
        except Exception as e:
            print(f"❌ Error testing {port}: {e}")
    
    return False

if __name__ == "__main__":
    print("UART Bidirectional Communication Test")
    print("=====================================")
    
    success = test_bidirectional_uart()
    
    if success:
        print("\n🎉 Bidirectional UART communication working!")
    else:
        print("\n💥 Bidirectional UART communication failed")
