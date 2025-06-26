#!/bin/bash

# UmmItOS Settings Build Script
# This script helps build and install the UmmItOS Settings application

set -e

echo "🔧 Building UmmItOS Settings..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust/Cargo is not installed. Please install Rust first."
    exit 1
fi

# Check if GTK4 development files are available
if ! pkg-config --exists gtk4; then
    echo "GTK4 development files not found. Please install them:"
    echo "   Arch Linux: sudo pacman -S gtk4 glib2"
    exit 1
fi

echo "Building application..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi 