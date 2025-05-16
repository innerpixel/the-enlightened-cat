# How The Enlightened Cat Works

This guide provides a high-level overview of how the different components of "The Enlightened Cat" work together.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                        User Browser                         │
│                                                             │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                      Nginx Web Server                       │
│                                                             │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                   Rust Axum Web Application                 │
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────────┐   │
│  │             │   │             │   │                 │   │
│  │  HTML Pages │   │  API Routes │   │  Static Assets  │   │
│  │             │   │             │   │                 │   │
│  └──────┬──────┘   └──────┬──────┘   └─────────────────┘   │
│         │                 │                                 │
│         ▼                 ▼                                 │
│  ┌─────────────┐   ┌─────────────┐                         │
│  │             │   │             │                         │
│  │  Templates  │   │ Application │                         │
│  │             │   │    Logic    │                         │
│  └─────────────┘   └──────┬──────┘                         │
│                           │                                 │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                       Mistral AI API                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Key Components and Flow

### 1. Frontend

The frontend consists of:
- HTML templates (using Askama templating engine)
- CSS for styling
- JavaScript for interactivity
- Static assets (images, etc.)

### 2. Backend

The Rust backend is built with Axum and handles:
- Serving HTML pages
- Processing API requests
- Managing application state
- Communicating with the Mistral AI API

### 3. AI Integration

The Mistral AI integration:
- Sends prompts to the Mistral API
- Processes responses
- Maintains conversation context
- Implements the Enlightened Cat's personality

## User Interaction Flow

1. **User visits the website**
   - Axum serves the HTML page using Askama templates
   - Static assets (CSS, JS, images) are served directly by Nginx

2. **User reads Daily Wisdom**
   - Backend checks if wisdom is cached and still valid
   - If needed, requests new wisdom from Mistral AI
   - Renders the wisdom on the page

3. **User chats with the Enlightened Cat**
   - User sends a message via the chat interface
   - JavaScript sends an AJAX request to the `/api/chat` endpoint
   - Backend forwards the message to Mistral AI with the cat's personality prompt
   - Response is returned to the frontend and displayed in the chat

## Code Structure Explained

### Main Components

1. **main.rs**
   - Entry point for the application
   - Sets up routes, middleware, and starts the server

2. **config.rs**
   - Manages environment variables and configuration

3. **mistral.rs**
   - Handles all communication with the Mistral AI API
   - Implements conversation management
   - Contains the Enlightened Cat's personality prompt

4. **state.rs**
   - Manages shared application state
   - Handles caching of daily wisdom

5. **routes/**
   - **chat.rs**: Handles chat API requests
   - **wisdom.rs**: Provides daily wisdom quotes
   - **pages.rs**: Renders HTML pages

6. **templates/**
   - HTML templates for the website

## Data Flow Examples

### Example 1: Getting Daily Wisdom

```
User visits /wisdom page
  │
  ▼
pages.rs handler is called
  │
  ▼
Handler requests wisdom from AppState
  │
  ▼
AppState checks if wisdom is cached and valid
  │
  ▼
If not valid, requests new wisdom from MistralClient
  │
  ▼
MistralClient sends request to Mistral API
  │
  ▼
Response is cached in AppState
  │
  ▼
Wisdom is rendered in the template
  │
  ▼
HTML is returned to the user
```

### Example 2: Chat Interaction

```
User sends message via chat interface
  │
  ▼
JavaScript sends AJAX request to /api/chat
  │
  ▼
chat.rs handler processes the request
  │
  ▼
Handler calls MistralClient.get_enlightened_cat_response()
  │
  ▼
MistralClient creates a conversation with:
  - System prompt defining the cat's personality
  - User's message
  │
  ▼
Request is sent to Mistral API
  │
  ▼
Response is processed and returned to frontend
  │
  ▼
JavaScript displays the response in the chat interface
```

## Key Files and Their Purpose

| File | Purpose |
|------|---------|
| `main.rs` | Application entry point and server setup |
| `config.rs` | Configuration management |
| `mistral.rs` | Mistral AI API integration |
| `state.rs` | Application state management |
| `routes/chat.rs` | Chat API endpoint handler |
| `routes/wisdom.rs` | Daily wisdom API endpoint handler |
| `routes/pages.rs` | HTML page handlers |
| `templates/base.html` | Base HTML template |
| `templates/index.html` | Home page template |
| `templates/about.html` | About page template |
| `templates/wisdom.html` | Wisdom page template |
| `static/css/styles.css` | CSS styles |
| `static/js/main.js` | Frontend JavaScript |
| `static/images/` | Image assets |

## Environment Variables

The application requires these environment variables:

- `MISTRAL_API_KEY`: Your Mistral AI API key
- `MISTRAL_API_URL`: Mistral API endpoint (default: https://api.mistral.ai/v1)
- `PORT`: Port for the web server (default: 3000)
- `RUST_LOG`: Logging level (default: info)

## Next Steps for Development

1. **Add User Authentication**
   - Allow users to create accounts
   - Save conversation history

2. **Expand Content Types**
   - Implement the "Purr-sonal Journeys" feature
   - Add more interactive elements

3. **Add Analytics**
   - Track popular wisdom quotes
   - Analyze common user questions

4. **Implement E-commerce**
   - Set up merchandise sales
   - Create premium subscription features
