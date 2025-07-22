#!/bin/bash

# KM-Box async-hid deployment script for Pi
# Run this on the Pi after cloning/pulling the repo

echo "=== KM-Box async-hid Deployment ==="

# Check if we are on the Pi
if [[ ! -f /etc/rpi-issue ]]; then
    echo "❌ This script must run on a Raspberry Pi"
    exit 1
fi

echo "✓ Running on Raspberry Pi"

# Navigate to the Pi code directory
cd ~/km-box/pi_code || {
    echo "❌ Could not find ~/km-box/pi_code directory"
    echo "Make sure you have cloned the repo to ~/km-box"
    exit 1
}

echo "✓ Found pi_code directory"

# Install Rust if not already installed
if ! command -v cargo &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

echo "✓ Rust is installed"

# Build the project
echo "🔨 Building async-hid implementation..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo ""
    echo "🚀 Ready to run! Use one of these commands:"
    echo ""
    echo "List all HID devices:"
    echo "  sudo ./target/release/km_pi --list-devices"
    echo ""
    echo "Run with SteelSeries mouse (default):"
    echo "  sudo ./target/release/km_pi --verbose"
    echo ""
    echo "Run with custom mouse VID/PID:"
    echo "  sudo ./target/release/km_pi --mouse-vid 0x1234 --mouse-pid 0x5678 --verbose"
    echo ""
    echo "💡 Remember: Run with sudo for HID device access!"
else
    echo "❌ Build failed!"
    echo "Check the error messages above and ensure all dependencies are installed."
    exit 1
fi
