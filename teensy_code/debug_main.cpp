#include <Arduino.h>

// UART Debug Mode - Show raw bytes received
void setup() {
  Serial.begin(115200);
  while (!Serial && millis() < 3000) {
    // Wait for Serial connection
  }
  
  Serial1.begin(9600);
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);
  
  Serial.println("=== KM-Box Teensy UART Debug Mode ===");
  Serial.println("Showing RAW BYTES received from Pi");
  Serial.println("Baud rate: 9600");
  Serial.println("Waiting for data...");
}

void loop() {
  // Check for data from Pi via UART
  if (Serial1.available()) {
    Serial.print("[UART DEBUG] Bytes available: ");
    Serial.println(Serial1.available());
    
    // Read and display each byte
    while (Serial1.available()) {
      int byte_received = Serial1.read();
      Serial.print("Byte: 0x");
      if (byte_received < 0x10) Serial.print("0");
      Serial.print(byte_received, HEX);
      Serial.print(" (");
      Serial.print(byte_received);
      Serial.print(") ");
      if (byte_received >= 32 && byte_received <= 126) {
        Serial.print("'");
        Serial.print((char)byte_received);
        Serial.print("'");
      } else {
        Serial.print("[non-printable]");
      }
      Serial.println();
    }
    Serial.println("--- End of data ---");
  }
  
  // Heartbeat every 5 seconds
  static unsigned long lastHeartbeat = 0;
  if (millis() - lastHeartbeat > 5000) {
    Serial.println("[HEARTBEAT] Debug mode active");
    lastHeartbeat = millis();
  }
  
  // LED blink
  static unsigned long lastBlink = 0;
  if (millis() - lastBlink > 1000) {
    digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
    lastBlink = millis();
  }
  
  delay(10);
}
