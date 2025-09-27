# Frame - Image Upload Service

This is designed to be run onboard a Raspberry Pi Zero.
It is responsible for both running the image software on an epaper display and enabling image upload via
a web client which is started upon a button press and terminated after a given amount of time to save
resources and respect power constraints.

A Rust web service built with Axum for handling image uploads for the photo frame.

## Features

- ✅ **Proper Error Handling**: Comprehensive error types and status codes
- ✅ **File Validation**: Validates file types and sizes before upload
- ✅ **Unique Filenames**: Prevents conflicts with timestamp-based naming
- ✅ **JSON Responses**: RESTful API with structured JSON responses
- ✅ **Modular Architecture**: Separated concerns with dedicated modules
- ✅ **Async File I/O**: Non-blocking file operations using Tokio
- ✅ **Memory Efficient**: Streams file data instead of loading into memory
- ✅ **Security**: File type validation and size limits

## API Endpoints

### GET /api/images
List all uploaded images.

### POST /api/images
Upload one or more image files.

## Configuration

- **Upload Directory:** `./uploads` (created automatically)
- **Max File Size:** 10MB
- **Allowed Extensions:** jpg, jpeg, png, gif, webp

## Running the Service

```bash
# Start the server
cargo run

# The server will listen on http://localhost:3000
```

## Usage

Manually with curl:

```bash
# List images
curl http://localhost:3000/api/images

# Upload an image
curl -X POST http://localhost:3000/api/images -F "image=@your_image.jpg"
```

### Architecture:

- `main.rs` - Application entry point
- `router.rs` - Route definitions
- `handlers.rs` - HTTP request handlers
- `file_store.rs` - File operations and validation logic

The code is now production-ready with proper error handling, validation, and a clean architecture.
