#!/bin/bash

# The Enlightened Cat - Deployment Script
# This script deploys the application to your VPS

echo "Deploying The Enlightened Cat to production..."

# 1. Build the release version
echo "Building release version..."
cargo build --release

# 2. Copy the systemd service file to the system
echo "Installing systemd service..."
sudo cp systemd/enlightened-cat.service /etc/systemd/system/
sudo systemctl daemon-reload

# 3. Copy the Nginx configuration
echo "Installing Nginx configuration..."
sudo cp nginx/enlightened-cat.conf /etc/nginx/sites-available/
sudo ln -sf /etc/nginx/sites-available/enlightened-cat.conf /etc/nginx/sites-enabled/

# 4. Test Nginx configuration
echo "Testing Nginx configuration..."
sudo nginx -t

if [ $? -ne 0 ]; then
    echo "Nginx configuration test failed. Please check the configuration."
    exit 1
fi

# 5. Restart Nginx
echo "Restarting Nginx..."
sudo systemctl restart nginx

# 6. Start or restart the service
echo "Starting The Enlightened Cat service..."
sudo systemctl enable enlightened-cat
sudo systemctl restart enlightened-cat

# 7. Show status
echo "Deployment complete! Service status:"
sudo systemctl status enlightened-cat

echo ""
echo "The Enlightened Cat is now running at https://the-enlightened-cat.com"
echo "You can check the logs with: sudo journalctl -u enlightened-cat -f"
