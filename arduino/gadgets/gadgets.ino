#include <Wire.h>
#include <LiquidCrystal.h>

#define I2C_ADDRESS 0x08

// initialize the library with the numbers of the interface pins
LiquidCrystal lcd(7, 8, 9, 10, 11, 12);
String lcdText = "";
int joyX = 0;
int joyY = 0;
int joyPressed = 0;

void setup() {
  Wire.begin(I2C_ADDRESS);
  Wire.onReceive(receiveEvent);
  Wire.onRequest(requestEvent);

  pinMode(2, INPUT_PULLUP);

  // set up the LCD's number of columns and rows:
  lcd.begin(16, 2);
  // Print a message to the LCD.
  lcd.print("Hello world.");
}

void loop() {
  joyX = analogRead(A0);
  joyY = analogRead(A1);
  joyPressed = digitalRead(2);

  if (lcdText.length() != 0) {
    lcd.clear();
    lcd.setCursor(0, 0);
    lcd.print(lcdText);
    lcdText = "";
  }

  delay(1);
}

// Called when Pico sends data to Arduino
void receiveEvent(int bytes) {
  lcdText = "";
  while (Wire.available()) {
    lcdText += (char)Wire.read();
  }
}

void requestEvent() {
  uint8_t data[5];
  data[0] = (joyX & 0xff00) >> 8;
  data[1] = joyX & 0xff;
  data[2] = (joyY & 0xff00) >> 8;
  data[3] = joyY & 0xff;
  data[4] = (joyPressed == LOW) ? 1 : 0;
  Wire.write(data, 5);
}

