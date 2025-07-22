# Teensy 4.0 Firmware - KM Box Project

## Overview
This is the Teensy 4.0 firmware component of the KM Box system, handling USB HID output and UART communication with the Raspberry Pi.

## Phase 2 - UART Communication ✅ TESTED

### Hardware Setup
- **Wiring**: Pi GPIO 14 (TX) → Teensy Pin 0 (RX1), Pi GPIO 15 (RX) → Teensy Pin 1 (TX1), GND → GND
- **Baud Rate**: 115200
- **Protocol**: Text-based commands with newline termination

### Supported Commands
- ping → Response: pong
- 	est → Response: TEENSY_UART_TEST_OK
- led_on → Response: LED_ON (turns on built-in LED)
- led_off → Response: LED_OFF (turns off built-in LED)  
- status → Response: System status with RAM and uptime
- heartbeat → Continuous status updates

### Test Results
✅ **Communication Verified**: Sub-millisecond latency, 100% success rate
✅ **LED Control**: Visual confirmation working
✅ **Data Integrity**: All byte counts accurate
✅ **Ready for Phase 3**: Input capture and HID output integration

For detailed implementation, see [PHASE_2_SUMMARY.md](PHASE_2_SUMMARY.md).

## Development
- **Framework**: Arduino/Teensyduino with PlatformIO
- **IDE**: VS Code with PlatformIO extension
- **Build**: pio run (or use VS Code tasks)
- **Upload**: pio run --target upload
- **Monitor**: pio device monitor

## Next Phase
Phase 3 will implement input capture on Pi and integrate with this UART communication system.
