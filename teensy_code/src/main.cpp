#include <Arduino.h>

void setup() {
  // Initialize USB keyboard functionality
  Keyboard.begin();
  
  // Initialize Serial1 for communication with Raspberry Pi
  Serial1.begin(115200);
  
  // Optional: Initialize USB Serial for debugging
  Serial.begin(115200);
  while (!Serial && millis() < 3000) {
    // Wait for Serial connection or timeout after 3 seconds
  }
  
  Serial.println("Teensy 4.0 KM-Box ready");
}

void loop() {
  // Check if data is available on Serial1
  if (Serial1.available()) {
    // Read character from Serial1
    char receivedChar = Serial1.read();
    
    // Type the character using USB keyboard
    Keyboard.write(receivedChar);
    
    // Optional: Echo to USB Serial for debugging
    Serial.print("Typed: ");
    Serial.println(receivedChar);
  }
  
  // Small delay to prevent overwhelming the system
  delay(1);
}
