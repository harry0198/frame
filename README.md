# Frame - Image Upload Service

This is designed to be run onboard a Raspberry Pi Zero.
It is responsible for both running the image software on an epaper display and enabling image upload via
a web client which is started upon a button press and terminated after a given amount of time to save
resources and respect power constraints.

Project contains:
- Hand rolled rust inky impression 7.3" driver (inky_e673)
- Rust web service built with Axum for handling image uploads for the photo frame.
- To be completed: egui interface for image upload and manipulation

## Driver supported actions
- Set pixel
- Set image
  - Dithering (floyd-steinberg algorithm to visually improve images)
  - Quantization (translate to 6 colours)

## API Endpoints

### GET /api/images
List all uploaded images.

### POST /api/images
Upload one or more image files.
