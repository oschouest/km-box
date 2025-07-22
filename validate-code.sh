#!/bin/bash
# Quick validation script for async-hid implementation

echo "=== KM-Box Async-HID Code Validation ==="

cd pi_code

echo "✓ Checking Cargo.toml dependencies..."
if grep -q "async-hid" Cargo.toml; then
    echo "  ✅ async-hid dependency found"
else 
    echo "  ❌ async-hid dependency missing"
fi

if grep -q "hex" Cargo.toml; then
    echo "  ✅ hex dependency found" 
else
    echo "  ❌ hex dependency missing"
fi

echo "✓ Checking main.rs implementation..."
if grep -q "HidBackend" src/main.rs; then
    echo "  ✅ HidBackend usage found"
else
    echo "  ❌ HidBackend usage missing"
fi

if grep -q "monitor_mouse_device" src/main.rs; then
    echo "  ✅ Mouse monitoring function found"
else
    echo "  ❌ Mouse monitoring function missing"  
fi

if grep -q "hex::encode" src/main.rs; then
    echo "  ✅ Hex encoding found"
else
    echo "  ❌ Hex encoding missing"
fi

echo "✓ Checking for old evdev code..."
if grep -q "evdev" src/main.rs; then
    echo "  ⚠️  Old evdev code still present - should be removed"
else
    echo "  ✅ No evdev code found (good!)"
fi

echo ""
echo "🚀 Ready to test on Pi!" 
echo "Run: bash ~/km-box/deploy-pi.sh"
