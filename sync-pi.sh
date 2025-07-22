#!/bin/bash
echo '=== Syncing km-box from GitHub ==='
cd ~/km-box
echo 'Pulling latest changes...'
git pull origin main
echo 'Building Rust project...'
cd pi_code
source ~/.cargo/env
cargo build --release
echo '✓ Sync complete! Binaries ready in target/release/'
echo ''
echo 'Available commands:'
echo '  sudo ./target/release/km_pi --list-devices'
echo '  sudo ./target/release/km_pi --verbose'
echo '  sudo ./target/release/hid_test'
