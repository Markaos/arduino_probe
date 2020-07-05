#define MODE_OFF     0
#define MODE_DIGITAL 1
#define MODE_ANALOG  2

byte mode[20];

void setup() {
  for (int i = 0; i < 20; i++) {
    mode[i] = MODE_OFF;
    pinMode(i, INPUT);
  }
  
  Serial.begin(115200);
  Serial.print("Arduino Probe ready\n");
}

void loop() {
  if(Serial.available()) {
    String command = Serial.readString();
    if(command.startsWith("pins ")) {
      command = command.substring(5);
      while(command.length() > 0) {
        int pin = strToPin(command.substring(0, command.indexOf('=')));
        mode[pin] = charToMode(command.charAt(command.indexOf('=') + 1));
        command = command.substring(command.indexOf('=') + 3);
      }
    } else if (command.startsWith("start")) {
      captureLoop();
    } else if (command.startsWith("dump")) {
      for(int i = 0; i < 20; i++) {
        Serial.print(mode[i]);
        Serial.print(' ');
      }
      Serial.print('\n');
    }
    Serial.print('\n');
    Serial.print("Ready\n");
  }
}

void captureLoop() {
  while(!Serial.available()) {
    bool first = true;
    for(int i = 0; i < 20; i++) {
      if(mode[i] == MODE_DIGITAL) {
        if(!first) Serial.print(',');
        first = false;
        Serial.print(digitalRead(i));
      } else if(mode[i] == MODE_ANALOG) {
        if(!first) Serial.print(',');
        first = false;
        Serial.print(analogRead(i));
      }
    }
    Serial.print('\n');
  }
}

byte charToMode(char mode) {
  switch (mode) {
    case 'a': return MODE_ANALOG;
    case 'd': return MODE_DIGITAL;
    case 'o': return MODE_OFF;
  }
}

int strToPin(String str) {
  if(str.equals("0")) {
    return 0;
  } else if(str.equals("1")) {
    return 1;
  } else if(str.equals("2")) {
    return 2;
  } else if(str.equals("3")) {
    return 3;
  } else if(str.equals("4")) {
    return 4;
  } else if(str.equals("5")) {
    return 5;
  } else if(str.equals("6")) {
    return 6;
  } else if(str.equals("7")) {
    return 7;
  } else if(str.equals("8")) {
    return 8;
  } else if(str.equals("9")) {
    return 9;
  } else if(str.equals("10")) {
    return 10;
  } else if(str.equals("11")) {
    return 11;
  } else if(str.equals("12")) {
    return 12;
  } else if(str.equals("13")) {
    return 13;
  } else if(str.equals("14")) {
    return 14;
  } else if(str.equals("15")) {
    return 15;
  } else if(str.equals("16")) {
    return 16;
  } else if(str.equals("17")) {
    return 17;
  } else if(str.equals("18")) {
    return 18;
  } else if(str.equals("19")) {
    return 19;
  } else if(str.equals("a0")) {
    return 14;
  } else if(str.equals("a1")) {
    return 15;
  } else if(str.equals("a2")) {
    return 16;
  } else if(str.equals("a3")) {
    return 17;
  } else if(str.equals("a4")) {
    return 18;
  } else if(str.equals("a5")) {
    return 19;
  } else return -1;
}
