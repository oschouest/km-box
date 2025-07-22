#!/bin/bash
echo '=== Setting up km-box Git workflow on Pi ==='

# Backup existing work
if [ -d ~/km-box ]; then
    echo 'Backing up existing km-box...'
    mv ~/km-box ~/km-box-backup-
fi

# Clone from GitHub
echo 'Cloning from GitHub...'
cd ~
git clone https://github.com/oschouest/km-box.git

# Build the project
echo 'Building Rust project...'
cd ~/km-box/pi_code
source ~/.cargo/env
cargo build --release

echo '✓ Setup complete!'
echo ''
echo 'Usage:'
echo '  ~/sync-pi.sh          # Sync and build latest code'
echo '  cd ~/km-box/pi_code'
echo '  sudo ./target/release/km_pi --list-devices'
echo '  sudo ./target/release/km_pi --verbose'
