#include <Arduino.h>

// Function declarations
void handleKeyEvent(String event);
void handleMouseEvent(String event);

void setup() {
  Serial.begin(115200);  // USB Serial for debugging
  Serial1.begin(9600);   // UART for Pi communication
  
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);
  
  Serial.println("[TEENSY] Phase 3: Input Event Handler");
  Serial.println("[TEENSY] Waiting for input events from Pi...");
}

void loop() {
  // Heartbeat
  static unsigned long lastHeartbeat = 0;
  if (millis() - lastHeartbeat > 5000) {
    Serial.println("[HEARTBEAT] Phase 3 active - awaiting input events");
    lastHeartbeat = millis();
  }
  
  // Check for incoming UART data from Pi
  if (Serial1.available()) {
    String command = Serial1.readStringUntil('\n');
    command.trim();
    
    Serial.printf("[UART] Received: '%s'\n", command.c_str());
    
    // Handle Phase 3 initialization
    if (command == "phase3_start") {
      Serial1.println("phase3_ready");
      Serial.println("[UART] Phase 3 initialization complete");
    }
    // Handle keyboard events: key:KEY_A:1 (press) or key:KEY_A:0 (release)
    else if (command.startsWith("key:")) {
      handleKeyEvent(command);
    }
    // Handle mouse events: mouse:REL_X:5 or mouse:REL_Y:-3
    else if (command.startsWith("mouse:")) {
      handleMouseEvent(command);
    }
    // Legacy Phase 2 commands (still supported)
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

void handleKeyEvent(String event) {
  // Parse: key:KEY_A:1 -> key = KEY_A, value = 1 (press) or 0 (release)
  int firstColon = event.indexOf(':', 4);  // Skip "key:"
  int secondColon = event.indexOf(':', firstColon + 1);
  
  if (firstColon != -1 && secondColon != -1) {
    String keyName = event.substring(4, firstColon);
    String valueStr = event.substring(secondColon + 1);
    int value = valueStr.toInt();
    
    Serial.printf("[INPUT] Key %s %s\n", 
                  keyName.c_str(), 
                  value == 1 ? "PRESSED" : (value == 0 ? "RELEASED" : "REPEAT"));
    
    // TODO Phase 4: Convert to USB HID keyboard output
    // For now, just acknowledge
    Serial1.println("key_processed");
  } else {
    Serial.printf("[ERROR] Invalid key event format: %s\n", event.c_str());
  }
}

void handleMouseEvent(String event) {
  // Parse: mouse:REL_X:5 -> axis = REL_X, value = 5
  int firstColon = event.indexOf(':', 6);  // Skip "mouse:"
  int secondColon = event.indexOf(':', firstColon + 1);
  
  if (firstColon != -1 && secondColon != -1) {
    String axisName = event.substring(6, firstColon);
    String valueStr = event.substring(secondColon + 1);
    int value = valueStr.toInt();
    
    Serial.printf("[INPUT] Mouse %s: %d\n", axisName.c_str(), value);
    
    // TODO Phase 4: Convert to USB HID mouse output
    // For now, just acknowledge
    Serial1.println("mouse_processed");
  } else {
    Serial.printf("[ERROR] Invalid mouse event format: %s\n", event.c_str());
  }
}
