#include <Arduino.h>
#include <Mouse.h>

// Global state tracking for buttons
uint8_t prev_buttons = 0;

// Function declarations
void handleHidReport(String hexData);
void sendMouseReport(uint8_t buttons, int16_t dx, int16_t dy, signed char wheel);

void setup() {
  Serial.begin(115200);  // USB Serial for debugging
  while (!Serial);       // Wait for serial
  Serial1.begin(115200); // UART for Pi communication (higher baud for low latency)
  
  Mouse.begin();         // Initialize HID mouse output
  
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);
  
  Serial.println("[TEENSY] Phase 5: Input Modification Framework");
  Serial.println("[TEENSY] USB HID Mouse output enabled");
  Serial.println("[TEENSY] Waiting for modified HID reports from Pi...");
}

void loop() {
  // Check for incoming UART data from Pi
  if (Serial1.available()) {
    String command = Serial1.readStringUntil('\n');
    command.trim();
    
    Serial.printf("[UART] Received: '%s'\n", command.c_str());
    
    if (command.startsWith("HID:")) {
      handleHidReport(command.substring(4));
    } else if (command == "INIT:PHASE5") {
      Serial.println("[UART] Phase 5 initialization acknowledged");
      Serial1.println("ack_phase5");
    } else {
      Serial.printf("[UART] Ignored non-HID: '%s'\n", command.c_str());
    }
  }
}

void handleHidReport(String hexData) {
  // Convert hex string to bytes
  int dataLength = hexData.length() / 2;
  if (dataLength != 9) {
    Serial.printf("[ERROR] HID report wrong length: %d bytes (expected 9)\n", dataLength);
    return;
  }
  
  uint8_t report[9] = {0};
  bool parseSuccess = true;
  
  for (int i = 0; i < 9; i++) {
    String byteString = hexData.substring(i * 2, i * 2 + 2);
    char* endPtr;
    long val = strtol(byteString.c_str(), &endPtr, 16);
    if (endPtr == byteString.c_str() || *endPtr != '\0') {  // Check for parse error
      Serial.printf("[ERROR] Hex parse failed at byte %d: '%s'\n", i, byteString.c_str());
      parseSuccess = false;
      break;
    }
    report[i] = (uint8_t)val;
  }
  
  if (!parseSuccess) return;
  
  // Debug: Print raw report bytes
  Serial.printf("[DEBUG] Raw report: %02x %02x %02x %02x %02x %02x %02x %02x %02x\n", 
                report[0], report[1], report[2], report[3], report[4], 
                report[5], report[6], report[7], report[8]);
  
  // Parse 9-byte HID report: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]
  uint8_t buttons = report[5];
  int16_t dx = (int16_t)((report[2] << 8) | report[1]);  // LE signed int16
  int16_t dy = (int16_t)((report[4] << 8) | report[3]);  // LE signed int16
  signed char wheel = (signed char)report[6];
  
  Serial.printf("[HID] Parsed: buttons=0x%02x dx=%d dy=%d wheel=%d\n", buttons, dx, dy, wheel);
  
  // Send USB HID mouse report
  sendMouseReport(buttons, dx, dy, wheel);
  
  Serial.println("[HID] Report processed and sent via USB");
  
  // Send acknowledgment back to Pi
  Serial1.println("hid_processed");
}

void sendMouseReport(uint8_t buttons, int16_t dx, int16_t dy, signed char wheel) {
  // Handle button state changes
  if ((buttons & 0x01) != (prev_buttons & 0x01)) {
    if (buttons & 0x01) {
      Mouse.press(MOUSE_LEFT);
      Serial.println("[USB] Left button pressed");
    } else {
      Mouse.release(MOUSE_LEFT);
      Serial.println("[USB] Left button released");
    }
  }
  
  if ((buttons & 0x02) != (prev_buttons & 0x02)) {
    if (buttons & 0x02) {
      Mouse.press(MOUSE_RIGHT);
      Serial.println("[USB] Right button pressed");
    } else {
      Mouse.release(MOUSE_RIGHT);
      Serial.println("[USB] Right button released");
    }
  }
  
  if ((buttons & 0x04) != (prev_buttons & 0x04)) {
    if (buttons & 0x04) {
      Mouse.press(MOUSE_MIDDLE);
      Serial.println("[USB] Middle button pressed");
    } else {
      Mouse.release(MOUSE_MIDDLE);
      Serial.println("[USB] Middle button released");
    }
  }
  
  // Handle side buttons (front = 0x08, back = 0x10)
  // Note: Arduino Mouse library doesn't have MOUSE_SIDE constants
  // For now, we'll log them but not send - user can remap later
  if ((buttons & 0x08) != (prev_buttons & 0x08)) {
    if (buttons & 0x08) {
      Serial.println("[USB] Front side button pressed (not mapped)");
    } else {
      Serial.println("[USB] Front side button released");
    }
  }
  
  if ((buttons & 0x10) != (prev_buttons & 0x10)) {
    if (buttons & 0x10) {
      Serial.println("[USB] Back side button pressed (not mapped)");
    } else {
      Serial.println("[USB] Back side button released");
    }
  }
  
  prev_buttons = buttons;
  
  // Handle movement with chunking for large deltas (but wheel should be sent once)
  signed char wheel_to_send = (signed char)max(-127, min(127, wheel));
  
  while (dx != 0 || dy != 0) {
    signed char dx_chunk = (signed char)max(-127, min(127, dx));
    signed char dy_chunk = (signed char)max(-127, min(127, dy));
    
    // Only send wheel data on first iteration
    signed char wheel_chunk = (wheel_to_send != 0) ? wheel_to_send : 0;
    wheel_to_send = 0;  // Clear after first send
    
    Mouse.move(dx_chunk, dy_chunk, wheel_chunk);
    Serial.printf("[USB] Mouse.move(%d, %d, %d)\n", dx_chunk, dy_chunk, wheel_chunk);
    
    dx -= dx_chunk;
    dy -= dy_chunk;
  }
  
  // If only wheel data (no movement), send it once
  if (wheel_to_send != 0) {
    Mouse.move(0, 0, wheel_to_send);
    Serial.printf("[USB] Mouse.move(0, 0, %d)\n", wheel_to_send);
  }
}
