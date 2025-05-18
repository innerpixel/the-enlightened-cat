#!/bin/bash

# The Enlightened Cat - Local Development Setup Script
# This script sets up a local development environment with Nginx

echo "Setting up local development environment for The Enlightened Cat..."

# 1. Add the domain to /etc/hosts if not already there
if ! grep -q "the-enlightened-cat.test" /etc/hosts; then
    echo "Adding the-enlightened-cat.test to /etc/hosts..."
    echo "127.0.0.1 the-enlightened-cat.test" | sudo tee -a /etc/hosts
else
    echo "Domain already in /etc/hosts"
fi

# 2. Copy the local Nginx configuration
echo "Installing local Nginx configuration..."
sudo cp nginx/enlightened-cat-local.conf /etc/nginx/sites-available/
sudo ln -sf /etc/nginx/sites-available/enlightened-cat-local.conf /etc/nginx/sites-enabled/

# 3. Test Nginx configuration
echo "Testing Nginx configuration..."
sudo nginx -t

if [ $? -ne 0 ]; then
    echo "Nginx configuration test failed. Please check the configuration."
    exit 1
fi

# 4. Restart Nginx
echo "Restarting Nginx..."
sudo systemctl restart nginx

# 5. Copy development environment to .env
echo "Setting up development environment variables..."
cp .env.development .env

echo ""
echo "Local development environment setup complete!"
echo "You can now access The Enlightened Cat at http://the-enlightened-cat.test"
echo "To start the application, run: ./run_dev.sh"
