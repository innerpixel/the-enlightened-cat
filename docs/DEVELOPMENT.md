# The Enlightened Cat - Development Guide

This guide will help you set up a local development environment for The Enlightened Cat project, understand the codebase structure, and contribute to this mindfulness web application.

## Project Overview

The Enlightened Cat is an interactive blog/adventure featuring an AI cat character that provides wisdom and engages with stressed urban professionals. The project aims to help people find balance and tranquility through interactive conversations, daily wisdom snippets, and micro-stories.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust and Cargo** (latest stable version)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Nginx** (for local domain setup)
  ```bash
  sudo apt install nginx
  ```

- **SSL Development Libraries** (required for HTTPS connections)
  ```bash
  sudo apt install pkg-config libssl-dev
  ```

- **Git** (for version control)
  ```bash
  sudo apt install git
  ```

- **Mistral API Key** (for AI interactions)
  - Sign up at [Mistral AI](https://mistral.ai)
  - Create an API key in your account dashboard

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/innerpixel/the-enlightened-cat.git
cd the-enlightened-cat
```

### 2. Set Up Environment Variables

Create a `.env.development` file in the project root with the following content:

```
MISTRAL_API_KEY=your_mistral_api_key_here
MISTRAL_API_URL=https://api.mistral.ai/v1
PORT=9000
RUST_LOG=debug
```

Replace `your_mistral_api_key_here` with your actual Mistral API key.

### 3. Set Up Local Development Environment

Run the setup script to configure your local environment:

```bash
./setup_local_dev.sh
```

This script will:
- Add `the-enlightened-cat.test` to your hosts file
- Install the local Nginx configuration
- Set up development environment variables

### 4. Run the Application

Start the application in development mode:

```bash
./run_dev.sh
```

The application will be available at:
- http://the-enlightened-cat.test
- http://localhost:9000

## Project Structure

```
the-enlightened-cat/
├── Cargo.toml             # Rust dependencies and project metadata
├── src/                   # Source code
│   ├── main.rs            # Application entry point
│   ├── config.rs          # Configuration management
│   ├── mistral.rs         # Mistral AI API client
│   ├── state.rs           # Application state management
│   ├── routes/            # HTTP route handlers
│   │   ├── mod.rs         # Module declaration
│   │   ├── chat.rs        # Chat API endpoints
│   │   ├── pages.rs       # HTML page rendering
│   │   └── wisdom.rs      # Daily wisdom API endpoints
│   └── templates/         # Askama HTML templates
├── templates/             # HTML templates
│   ├── base.html          # Base template with common layout
│   ├── index.html         # Home page template
│   ├── about.html         # About page template
│   └── wisdom.html        # Daily wisdom page template
├── static/                # Static assets
│   ├── css/               # CSS stylesheets
│   ├── js/                # JavaScript files
│   └── images/            # Image assets
├── nginx/                 # Nginx configurations
│   ├── enlightened-cat.conf        # Production configuration
│   └── enlightened-cat-local.conf  # Local development configuration
├── systemd/               # Systemd service files
│   └── enlightened-cat.service     # Production service configuration
├── .env.development       # Development environment variables
├── .gitignore             # Git ignore patterns
├── run_dev.sh             # Script to run in development mode
├── setup_local_dev.sh     # Script to set up local environment
└── deploy.sh              # Script to deploy to production
```

## Key Components

### Rust Web Application

- **Axum**: Modern web framework for handling HTTP requests
- **Tokio**: Async runtime for non-blocking operations
- **Askama**: HTML templating engine for rendering pages
- **Serde**: Serialization/deserialization for JSON handling

### State Management

The application uses a shared state pattern (similar to Redux/Zustand in frontend frameworks):

- `AppState` in `state.rs` manages shared resources
- Thread-safe access using `Arc` and `RwLock`
- Caching mechanism for daily wisdom to reduce API calls

### Mistral AI Integration

- `MistralClient` in `mistral.rs` handles communication with the Mistral API
- Generates wisdom quotes and chat responses
- Configurable prompts for different types of content

## Development Workflow

1. **Make Changes**: Modify code, templates, or static assets
2. **Test Locally**: Run with `./run_dev.sh` and test at http://the-enlightened-cat.test
3. **Commit Changes**: Use descriptive commit messages
4. **Push to GitHub**: Share your changes with the team

## Adding New Features

### Adding a New Page

1. Create a new HTML template in `templates/`
2. Add a route handler in `src/routes/pages.rs`
3. Register the route in `src/main.rs`

### Adding a New API Endpoint

1. Create a handler function in the appropriate route file
2. Define request/response structures with Serde
3. Register the route in `src/main.rs`

### Modifying the Daily Wisdom

The Daily Whispurr is generated using a prompt in `src/mistral.rs`. To modify its style or content:

1. Edit the `get_daily_wisdom` method in `src/mistral.rs`
2. Update the system message to change the tone, style, or content guidelines
3. Test locally to see the new wisdom format

## Deployment

When you're ready to deploy to production:

1. Push your changes to GitHub
2. SSH into the production server
3. Pull the latest changes
4. Run `./deploy.sh`

The deployment script will:
- Build the release version
- Install the systemd service
- Configure Nginx
- Restart necessary services

## Troubleshooting

### Common Issues

- **Compilation Errors**: Check that you have the latest Rust version and all dependencies
- **API Connection Issues**: Verify your Mistral API key and internet connection
- **Nginx Configuration**: Ensure Nginx is properly installed and configured

### Logs

- **Application Logs**: Check the console output when running with `./run_dev.sh`
- **Production Logs**: Use `sudo journalctl -u enlightened-cat -f` on the server

## Contributing

We welcome contributions to The Enlightened Cat! Here are some ways you can help:

- **Code**: Implement new features or fix bugs
- **Design**: Improve the UI/UX or create new visual assets
- **Content**: Suggest new prompts or wisdom themes
- **Documentation**: Enhance this guide or add code comments

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contact

For questions or suggestions, please open an issue on GitHub or contact the project maintainer.

---

May The Enlightened Cat bring mindfulness and tranquility to your coding journey! 🐱✨
