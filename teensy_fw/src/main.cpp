#include <Arduino.h>

void setup() {
  // Initialize built-in LED for visual feedback
  pinMode(LED_BUILTIN, OUTPUT);
  
  // Initialize USB Serial for debugging
  Serial.begin(115200);
  while (!Serial && millis() < 3000) {
    // Wait for Serial connection or timeout after 3 seconds
  }
  
  Serial.println("Hello from Teensy 4.0!");
  Serial.println("KM-Box Phase 1 Test - LED Blink");
}

void loop() {
  // Blink LED and print message every 2 seconds
  digitalWrite(LED_BUILTIN, HIGH);
  Serial.println("LED ON - Teensy is alive!");
  delay(1000);
  
  digitalWrite(LED_BUILTIN, LOW);
  Serial.println("LED OFF");
  delay(1000);
}
