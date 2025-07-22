#include <Arduino.h>

// Phase 2: UART Communication Test
// Hardware Setup:
// Pi GPIO 14 (TX) -> Teensy RX1 (Pin 0) 
// Pi GPIO 15 (RX) -> Teensy TX1 (Pin 1)
// Pi GND -> Teensy GND

void setup() {
  // Initialize USB Serial for debugging
  Serial.begin(115200);
  while (!Serial && millis() < 3000) {
    // Wait for Serial connection or timeout
  }
  
  // Initialize Serial1 for UART communication with Pi
  Serial1.begin(115200);
  
  // Built-in LED for status indication
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);
  
  Serial.println("=== KM-Box Teensy Phase 2: UART Communication ===");
  Serial.println("Hardware: Pi TX(GPIO14)->Teensy RX1(Pin0), Pi RX(GPIO15)->Teensy TX1(Pin1)");
  Serial.println("Waiting for commands from Pi via Serial1...");
  Serial.println("Commands: 'ping', 'led_on', 'led_off', 'test', 'status'");
  Serial.println("Baud rate: 115200");
  
  // Send initial ready message to Pi
  Serial1.println("teensy_ready");
}

void loop() {
  // Check for data from Pi via UART
  if (Serial1.available()) {
    String command = Serial1.readStringUntil('\n');
    command.trim();
    
    Serial.print("[UART RX] From Pi: '");
    Serial.print(command);
    Serial.println("'");
    
    // Process commands from Pi
    if (command == "ping") {
      Serial1.println("pong");
      Serial.println("[UART TX] Sent: pong");
    }
    else if (command == "led_on") {
      digitalWrite(LED_BUILTIN, HIGH);
      Serial1.println("led_on_ok");
      Serial.println("[LED] ON - Sent: led_on_ok");
    }
    else if (command == "led_off") {
      digitalWrite(LED_BUILTIN, LOW);
      Serial1.println("led_off_ok");
      Serial.println("[LED] OFF - Sent: led_off_ok");
    }
    else if (command == "test") {
      Serial1.println("teensy_test_ok");
      Serial.println("[TEST] Sent: teensy_test_ok");
    }
    else if (command == "status") {
      Serial1.print("status_ok,led:");
      Serial1.print(digitalRead(LED_BUILTIN) ? "on" : "off");
      Serial1.print(",uptime:");
      Serial1.print(millis());
      Serial1.println("ms");
      Serial.println("[STATUS] Sent status info");
    }
    else if (command.length() > 0) {
      Serial1.println("unknown_command");
      Serial.print("[ERROR] Unknown command: '");
      Serial.print(command);
      Serial.println("' - Sent: unknown_command");
    }
  }
  
  // Heartbeat LED blink every 2 seconds when no Pi communication
  static unsigned long lastBlink = 0;
  static unsigned long lastHeartbeat = 0;
  
  if (millis() - lastHeartbeat > 5000) {  // 5 second heartbeat
    Serial1.println("heartbeat");
    Serial.println("[HEARTBEAT] Sent to Pi");
    lastHeartbeat = millis();
  }
  
  if (millis() - lastBlink > 1000) {  // 1 second blink
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
    lastBlink = millis();
  }
  
  delay(10); // Small delay to prevent overwhelming
}
