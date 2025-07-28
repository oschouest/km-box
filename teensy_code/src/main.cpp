#include <Arduino.h>
#include <Mouse.h>

// Global state tracking for buttons
uint8_t prev_buttons = 0;

// Function declarations
void handleHidReport(String hexData);
void sendMouseReport(uint8_t buttons, int16_t dx, int16_t dy, signed char wheel);

void setup() {
  Serial.begin(115200);  // USB Serial for debugging
  Serial1.begin(9600);   // UART for Pi communication
  
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);
  
  Serial.println("[TEENSY] Phase 5: Input Modification Framework");
  Serial.println("[TEENSY] USB HID Mouse output enabled");
  Serial.println("[TEENSY] Waiting for modified HID reports from Pi...");
}

void loop() {
  // Heartbeat (reduced frequency for production)
  static unsigned long lastHeartbeat = 0;
  if (millis() - lastHeartbeat > 10000) {
    Serial.println("[HEARTBEAT] Phase 5 active - USB HID passthrough ready");
    lastHeartbeat = millis();
  }
  
  // Check for incoming UART data from Pi
  if (Serial1.available()) {
    String command = Serial1.readStringUntil('\n');
    command.trim();
    
    Serial.printf("[UART] Received: '%s'\n", command.c_str());
    
    // Handle Phase 5 initialization
    if (command == "phase5_start") {
      Serial1.println("phase5_ready");
      Serial.println("[UART] Phase 5 initialization complete - USB HID ready");
      digitalWrite(LED_BUILTIN, HIGH); // Indicate ready
    }
    // Handle HID reports: HID:001234abcd
    else if (command.startsWith("HID:")) {
      String hexData = command.substring(4);
      handleHidReport(hexData);
    }
    // Legacy commands (still supported)
    else if (command == "ping") {
      Serial1.println("pong");
      Serial.println("[UART] Sent: pong");
    }
    else if (command == "led_on") {
      digitalWrite(LED_BUILTIN, HIGH);
      Serial1.println("led_on_ok");
      Serial.println("[UART] LED ON, Sent: led_on_ok");
    }
    else if (command == "led_off") {
      digitalWrite(LED_BUILTIN, LOW);
      Serial1.println("led_off_ok");
      Serial.println("[UART] LED OFF, Sent: led_off_ok");
    }
    else if (command == "status") {
      bool ledState = digitalRead(LED_BUILTIN);
      String response = "led_" + String(ledState ? "on" : "off");
      Serial1.println(response);
      Serial.printf("[UART] Sent: %s\n", response.c_str());
    }
    else if (command == "test") {
      Serial1.println("test_ok");
      Serial.println("[UART] Sent: test_ok");
    }
    else {
      Serial1.println("unknown_command");
      Serial.printf("[UART] Unknown command: '%s'\n", command.c_str());
    }
    
    Serial1.flush(); // Ensure data is sent
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
  
  for (int i = 0; i < 9; i++) {
    String byteString = hexData.substring(i * 2, i * 2 + 2);
    report[i] = (uint8_t)strtol(byteString.c_str(), NULL, 16);
  }
  
  // Parse 9-byte HID report: [00, dx_low, dx_high, dy_low, dy_high, buttons, wheel, 00, 00]
  uint8_t buttons = report[5];
  int16_t dx = (report[2] << 8) | report[1];  // Little-endian int16
  int16_t dy = (report[4] << 8) | report[3];  // Little-endian int16
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
  
  prev_buttons = buttons;
  
  // Handle movement with chunking for large deltas
  while (dx != 0 || dy != 0 || wheel != 0) {
    signed char dx_chunk = (signed char)max(-127, min(127, dx));
    signed char dy_chunk = (signed char)max(-127, min(127, dy));
    signed char wheel_chunk = (signed char)max(-127, min(127, wheel));
    
    Mouse.move(dx_chunk, dy_chunk, wheel_chunk);
    Serial.printf("[USB] Mouse.move(%d, %d, %d)\n", dx_chunk, dy_chunk, wheel_chunk);
    
    dx -= dx_chunk;
    dy -= dy_chunk;
    wheel -= wheel_chunk;
  }
}
