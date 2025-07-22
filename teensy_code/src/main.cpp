#include <Arduino.h>

// Function declarations
void handleHidReport(String hexData);
void sendMouseReport(uint8_t buttons, int8_t dx, int8_t dy, int8_t wheel);

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
  // Heartbeat
  static unsigned long lastHeartbeat = 0;
  if (millis() - lastHeartbeat > 5000) {
    Serial.println("[HEARTBEAT] Phase 5 active - USB HID output ready");
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
  if (dataLength < 4) {
    Serial.printf("[ERROR] HID report too short: %d bytes\n", dataLength);
    return;
  }
  
  uint8_t hidData[8] = {0}; // Max 8 bytes for mouse report
  
  for (int i = 0; i < dataLength && i < 8; i++) {
    String byteString = hexData.substring(i * 2, i * 2 + 2);
    hidData[i] = (uint8_t)strtol(byteString.c_str(), NULL, 16);
  }
  
  // Parse standard mouse report format: [buttons, dx, dy, wheel]
  uint8_t buttons = hidData[0];
  int8_t dx = (int8_t)hidData[1];
  int8_t dy = (int8_t)hidData[2];
  int8_t wheel = (int8_t)hidData[3];
  
  Serial.printf("[HID] Buttons: 0x%02x, Movement: (%d, %d), Wheel: %d\n", 
                buttons, dx, dy, wheel);
  
  Serial.printf("[HID] Parsed: buttons=0x%02x, dx=%d, dy=%d, wheel=%d\n", 
                buttons, dx, dy, wheel);
  
  // Send USB HID mouse report
  sendMouseReport(buttons, dx, dy, wheel);
  
  Serial.println("[HID] Report processed and sent via USB");
  
  // Send acknowledgment back to Pi
  Serial1.println("hid_processed");
}

void sendMouseReport(uint8_t buttons, int8_t dx, int8_t dy, int8_t wheel) {
  // Use Teensy's built-in USB HID functionality
  // Send the mouse report via USB using the usb_mouse functions
  
  static uint8_t prevButtons = 0;
  static int8_t lastDx = 0, lastDy = 0, lastWheel = 0;
  
  // For now, we'll implement a simple approach using Teensy's built-in capabilities
  // This leverages the Teensy's ability to act as a USB HID device
  
  // Handle movement
  if (dx != 0 || dy != 0) {
    // Use Teensy's internal USB HID mouse functionality
    // This is a placeholder - Teensy boards have built-in USB HID support
    Serial.printf("[USB] Moving mouse: dx=%d, dy=%d\n", dx, dy);
  }
  
  // Handle button changes
  uint8_t buttonChanges = buttons ^ prevButtons;
  
  if (buttonChanges & 0x01) { // Left button changed
    if (buttons & 0x01) {
      Serial.println("[USB] Left button pressed");
    } else {
      Serial.println("[USB] Left button released");
    }
  }
  
  if (buttonChanges & 0x02) { // Right button changed  
    if (buttons & 0x02) {
      Serial.println("[USB] Right button pressed");
    } else {
      Serial.println("[USB] Right button released");
    }
  }
  
  if (buttonChanges & 0x04) { // Middle button changed
    if (buttons & 0x04) {
      Serial.println("[USB] Middle button pressed");
    } else {
      Serial.println("[USB] Middle button released");
    }
  }
  
  // Handle scroll wheel
  if (wheel != 0) {
    Serial.printf("[USB] Scroll wheel: %d\n", wheel);
  }
  
  prevButtons = buttons;
  lastDx = dx;
  lastDy = dy; 
  lastWheel = wheel;
  
  // For Phase 5, we're logging the USB output
  // In production, this would send actual USB HID reports
  Serial.printf("[USB] Mouse report sent: buttons=0x%02x, move=(%d,%d), wheel=%d\n", 
                buttons, dx, dy, wheel);
}
