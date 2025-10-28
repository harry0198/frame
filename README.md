

# PINS
The Witty PI 4 Mini and Inky Impression are incompatible because they have GPIO pins which overlap. Namely, GPIO 17 (pin 11) - the inky's BUSY pin. As I opted to write the inky impression library in rust, this project has easy access to updating which pins it should interact with for the inky impression.
Therefore, it's simply a case of moving the GPIO 17 wire on the pi zero to GPIO 23 or similar (but sneakily keeping it on GPIO 17 on the inky impression). This means that as far as the inky circuit board is concerned, everything is peachy. This way, I can just update the inky library code to assume the BUSY pin is on GPIO23. 

The inky pins:
https://pinout.xyz/pinout/inky_impression