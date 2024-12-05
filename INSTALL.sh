#!/bin/bash

if [ "$EUID" -ne 0 ]; then
  echo "you must run this script as root."
  exit
fi

echo "Checking dependencies..."
if ! command -v cargo &> /dev/null; then
  echo "Cargo was not detected. Please install Rust and Cargo."
  exit 1
fi

echo "Building the project in release mode..."
if ! cargo build --release; then
  echo "Build failed. Fix the errors and try again."
  exit 1
fi

TARGET_PATH="./target/release/brsc"

if [ ! -f "$TARGET_PATH" ]; then
  echo "Binary not found at $TARGET_PATH. Build might have failed."
  exit 1
fi

echo "Installing $BIN_NAME to /bin..."
cp "$TARGET_PATH" /bin/
chmod +x /bin/brsc

echo "Installation complete! You can run the program with 'brsc'."
