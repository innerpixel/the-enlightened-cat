# The Enlightened Cat - Technical Documentation

This document provides a comprehensive overview of the technical implementation, architecture, and deployment process for "The Enlightened Cat" project.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Directory Structure](#directory-structure)
4. [Key Components](#key-components)
5. [Development Setup](#development-setup)
6. [Deployment Guide](#deployment-guide)
7. [API Documentation](#api-documentation)
8. [Troubleshooting](#troubleshooting)

## Project Overview

"The Enlightened Cat" is an interactive web application featuring an AI-powered cat character that provides wisdom and guidance to stressed professionals. The application combines:

- A Rust-based backend using Axum web framework
- Mistral AI integration for generating the cat's responses
- HTML templates with Askama templating engine
- Static assets for a zen-inspired UI

The project aims to create an engaging, conversational experience while maintaining a calming, mindfulness-focused aesthetic.

## Architecture

The application follows a layered architecture:

```
User Interface (HTML/CSS/JS)
        ↓
Web Server (Axum)
        ↓
Application Logic
        ↓
Mistral AI Integration
```

### Key Flow:

1. User visits the website or interacts with the chat
2. Request is handled by the Axum web server
3. Application logic processes the request
4. For AI responses, the Mistral API is called
5. Response is rendered using HTML templates
6. User receives the response

## Directory Structure

```
the-enlightened-cat/
├── Cargo.toml              # Rust dependencies and project metadata
├── src/                    # Rust source code
│   ├── main.rs             # Application entry point
│   ├── config.rs           # Configuration management
│   ├── mistral.rs          # Mistral API integration
│   ├── state.rs            # Application state management
│   ├── routes/             # HTTP route handlers
│   │   ├── mod.rs          # Routes module definition
│   │   ├── chat.rs         # Chat API endpoints
│   │   ├── pages.rs        # HTML page handlers
│   │   └── wisdom.rs       # Daily wisdom endpoints
│   └── templates/          # Rust template definitions
│       └── mod.rs          # Template structs
├── templates/              # HTML templates
│   ├── base.html           # Base template with common elements
│   ├── index.html          # Home page template
│   ├── about.html          # About page template
│   └── wisdom.html         # Daily wisdom page template
├── static/                 # Static assets
│   ├── css/                # Stylesheets
│   ├── js/                 # JavaScript files
│   └── images/             # Image assets
├── .env                    # Environment variables (not in repo)
└── README.md               # Project overview
```

## Key Components

### 1. Web Server (main.rs)

The entry point for the application that:
- Initializes the configuration
- Sets up logging
- Creates the application state
- Defines the routes
- Starts the web server

### 2. Configuration (config.rs)

Manages application configuration using environment variables:
- Mistral API key
- API endpoints
- Server port

### 3. Mistral API Integration (mistral.rs)

Handles communication with the Mistral AI API:
- Sends chat messages to the API
- Processes responses
- Manages conversation context
- Implements specific prompts for the Enlightened Cat

### 4. Application State (state.rs)

Manages shared application state:
- Mistral client instance
- Daily wisdom caching
- Last update timestamps

### 5. Route Handlers (routes/)

Process HTTP requests:
- `chat.rs`: Handles chat API requests
- `wisdom.rs`: Provides daily wisdom quotes
- `pages.rs`: Renders HTML pages

### 6. Templates

Two levels of templates:
- Rust template definitions (`src/templates/mod.rs`)
- HTML templates (`templates/`)

## Development Setup

### Prerequisites

- Rust (1.56 or later)
- Cargo
- Mistral API key

### Environment Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd the-enlightened-cat
   ```

2. Create a `.env` file in the project root:
   ```
   MISTRAL_API_KEY=your_mistral_api_key
   MISTRAL_API_URL=https://api.mistral.ai/v1
   PORT=3000
   RUST_LOG=info
   ```

3. Install dependencies and build:
   ```bash
   cargo build
   ```

4. Run the development server:
   ```bash
   cargo run
   ```

5. Access the application at `http://localhost:3000`

## Deployment Guide

### VPS Deployment

1. **Prepare the server**:
   - Ensure Rust is installed on your VPS
   - Set up Nginx as a reverse proxy

2. **Clone and build the application**:
   ```bash
   git clone <repository-url> /var/www/the-enlightened-cat
   cd /var/www/the-enlightened-cat
   cargo build --release
   ```

3. **Set up environment variables**:
   Create a `.env` file with production settings.

4. **Create a systemd service**:
   Create `/etc/systemd/system/enlightened-cat.service`:
   ```
   [Unit]
   Description=The Enlightened Cat Web Service
   After=network.target

   [Service]
   User=www-data
   WorkingDirectory=/var/www/the-enlightened-cat
   Environment="RUST_LOG=info"
   EnvironmentFile=/var/www/the-enlightened-cat/.env
   ExecStart=/var/www/the-enlightened-cat/target/release/the-enlightened-cat
   Restart=always

   [Install]
   WantedBy=multi-user.target
   ```

5. **Start the service**:
   ```bash
   sudo systemctl enable enlightened-cat
   sudo systemctl start enlightened-cat
   ```

6. **Configure Nginx**:
   Create `/etc/nginx/sites-available/the-enlightened-cat`:
   ```nginx
   server {
       listen 80;
       server_name the-enlightened-cat.com www.the-enlightened-cat.com;

       location / {
           proxy_pass http://127.0.0.1:3000;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
           proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
           proxy_set_header X-Forwarded-Proto $scheme;
       }

       location /static {
           alias /var/www/the-enlightened-cat/static;
           expires 1d;
       }
   }
   ```

7. **Enable the site and restart Nginx**:
   ```bash
   sudo ln -s /etc/nginx/sites-available/the-enlightened-cat /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl restart nginx
   ```

8. **Set up SSL with Let's Encrypt**:
   ```bash
   sudo apt install certbot python3-certbot-nginx
   sudo certbot --nginx -d the-enlightened-cat.com -d www.the-enlightened-cat.com
   ```

## API Documentation

### Chat API

**Endpoint**: `POST /api/chat`

**Request Body**:
```json
{
  "message": "How can I find more balance in my work life?"
}
```

**Response**:
```json
{
  "message": "In the corporate jungle, balance is not found but created. Set clear boundaries between work and rest—perhaps a small ritual that marks the transition. I've observed humans who thrive when they dedicate even 5 minutes to stillness before entering their home. What small boundary might you create for yourself today?"
}
```

### Daily Wisdom API

**Endpoint**: `GET /api/daily-wisdom`

**Response**:
```json
{
  "wisdom": "Like a cat who knows when to play and when to rest, your productivity depends not on constant motion, but on the rhythm between effort and renewal.",
  "timestamp": "2025-05-16T19:28:13.123Z"
}
```

## Troubleshooting

### Common Issues

1. **API Key Issues**:
   - Ensure your Mistral API key is correctly set in the `.env` file
   - Check that the API key has sufficient permissions

2. **Server Won't Start**:
   - Verify the port is not in use by another application
   - Check the logs with `journalctl -u enlightened-cat`

3. **Template Rendering Errors**:
   - Ensure all template files exist in the correct locations
   - Check for syntax errors in the templates

4. **Static Files Not Loading**:
   - Verify the static directory exists and has the correct permissions
   - Check Nginx configuration for the static file location

### Logging

The application uses the `tracing` crate for logging. To increase log verbosity, set:

```
RUST_LOG=debug
```

Or for even more detailed logs:

```
RUST_LOG=trace
```

---

For additional support or questions, please contact the development team.
