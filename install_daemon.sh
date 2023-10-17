#!/bin/bash

# Step 1: Build the project
cargo build --release
if [ $? -ne 0 ]; then
    echo "Cargo build failed. Exiting."
    exit 1
fi

# Step 2: Create the systemd service file for the daemon
SERVICE_FILE="/etc/systemd/system/gnome_auto_dark.service"
cat > "${SERVICE_FILE}" <<EOL
[Unit]
Description=GNOME Auto Dark Daemon
After=network.target

[Service]
ExecStart=$(pwd)/target/release/gnome_auto_dark
Restart=always
User=$(whoami)
WorkingDirectory=$(pwd)
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
EOL

# Reload systemd and enable the service
sudo systemctl daemon-reload
sudo systemctl enable gnome_auto_dark
sudo systemctl start gnome_auto_dark

# Check the status
sudo systemctl status gnome_auto_dark