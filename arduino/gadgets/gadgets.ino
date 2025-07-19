#include <Wire.h>
#include <LiquidCrystal.h>

#define I2C_ADDRESS 0x08

// initialize the library with the numbers of the interface pins
LiquidCrystal lcd(7, 8, 9, 10, 11, 12);
String lcdText = "";

void setup() {
  Wire.begin(I2C_ADDRESS);
  Wire.onReceive(receiveEvent);
  Wire.onRequest(requestEvent);

  // set up the LCD's number of columns and rows:
  lcd.begin(16, 2);
  // Print a message to the LCD.
  lcd.print("Hello world.");
}

void loop() {
  if (lcdText.length() != 0) {
    lcd.setCursor(0, 1);
    lcd.print(lcdText);
    lcdText = "";
  }
}

// Called when Pico sends data to Arduino
void receiveEvent(int bytes) {
  lcdText = "";
  while (Wire.available()) {
    lcdText += (char)Wire.read();
  }
}

void requestEvent() {
  
}

