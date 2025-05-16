# Deployment Guide for The Enlightened Cat

This guide provides step-by-step instructions for deploying "The Enlightened Cat" to your VPS with Nginx.

## Prerequisites

- A VPS with Ubuntu/Debian
- Domain name pointing to your VPS (the-enlightened-cat.com)
- Mistral API key

## 1. Server Setup

### Install Required Packages

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential curl git nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Set Up Firewall (Optional but Recommended)

```bash
sudo apt install -y ufw
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https
sudo ufw enable
```

## 2. Application Deployment

### Clone the Repository

```bash
sudo mkdir -p /var/www
cd /var/www
sudo git clone https://github.com/innerpixel/the-enlightened-cat.git
sudo chown -R $USER:$USER /var/www/the-enlightened-cat
cd the-enlightened-cat
```

### Create Static Directories

```bash
mkdir -p static/css
mkdir -p static/js
mkdir -p static/images
```

### Create Essential Static Files

#### CSS File (static/css/styles.css)

Create a file at `/var/www/the-enlightened-cat/static/css/styles.css` with your styles. Here's a minimal example:

```css
:root {
  --color-primary: #5a7d7c;
  --color-secondary: #dadada;
  --color-accent: #ff9e7d;
  --color-text: #333333;
  --color-bg: #f7f9f9;
  --font-heading: 'Lora', serif;
  --font-body: 'Open Sans', sans-serif;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: var(--font-body);
  color: var(--color-text);
  background-color: var(--color-bg);
  line-height: 1.6;
}

.container {
  width: 90%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

/* Header Styles */
header {
  background-color: white;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  padding: 15px 0;
}

header .container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo a {
  font-family: var(--font-heading);
  font-size: 1.8rem;
  color: var(--color-primary);
  text-decoration: none;
  font-weight: 600;
}

nav ul {
  display: flex;
  list-style: none;
}

nav ul li {
  margin-left: 30px;
}

nav ul li a {
  text-decoration: none;
  color: var(--color-text);
  font-weight: 400;
  transition: color 0.3s ease;
}

nav ul li a:hover {
  color: var(--color-primary);
}

/* Chat Interface */
#chat-container {
  position: fixed;
  bottom: 20px;
  right: 20px;
  width: 350px;
  height: 500px;
  background-color: white;
  border-radius: 10px;
  box-shadow: 0 5px 25px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  z-index: 1000;
  transition: all 0.3s ease;
}

#chat-container.hidden {
  transform: translateY(calc(100% + 20px));
  opacity: 0;
}

#chat-header {
  background-color: var(--color-primary);
  color: white;
  padding: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

#chat-header h3 {
  margin: 0;
  font-weight: 400;
}

#close-chat {
  background: none;
  border: none;
  color: white;
  font-size: 1.5rem;
  cursor: pointer;
}

#chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 15px;
  display: flex;
  flex-direction: column;
}

.message {
  display: flex;
  margin-bottom: 15px;
  max-width: 80%;
}

.cat-message {
  align-self: flex-start;
}

.user-message {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.avatar {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  margin-right: 10px;
  font-size: 1.2rem;
}

.user-message .avatar {
  margin-right: 0;
  margin-left: 10px;
  background-color: var(--color-accent);
}

.message-content {
  background-color: #f0f0f0;
  padding: 10px 15px;
  border-radius: 18px;
}

.cat-message .message-content {
  background-color: #e6f2f2;
}

.user-message .message-content {
  background-color: var(--color-primary);
  color: white;
}

#chat-input-container {
  display: flex;
  padding: 10px;
  border-top: 1px solid #eee;
}

#chat-input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 20px;
  outline: none;
}

#send-message {
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 20px;
  padding: 10px 15px;
  margin-left: 10px;
  cursor: pointer;
}

#open-chat {
  position: fixed;
  bottom: 20px;
  right: 20px;
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 30px;
  padding: 12px 20px;
  cursor: pointer;
  font-size: 1rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 999;
  transition: all 0.3s ease;
}

#open-chat.hidden {
  transform: scale(0);
  opacity: 0;
}

/* Hero Section */
.hero {
  display: flex;
  align-items: center;
  padding: 60px 0;
  min-height: 60vh;
}

.cat-illustration {
  flex: 1;
  display: flex;
  justify-content: center;
}

.cat-illustration img {
  max-width: 100%;
  height: auto;
  max-height: 400px;
}

.hero-content {
  flex: 1;
  padding: 0 20px;
}

h1 {
  font-family: var(--font-heading);
  font-size: 3rem;
  margin-bottom: 20px;
  color: var(--color-text);
}

.highlight {
  color: var(--color-primary);
}

.tagline {
  font-size: 1.2rem;
  margin-bottom: 30px;
  color: #666;
}

.cta-buttons {
  display: flex;
  gap: 15px;
}

.cta-button {
  padding: 12px 25px;
  border-radius: 30px;
  font-weight: 600;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
}

.primary {
  background-color: var(--color-primary);
  color: white;
  border: none;
}

.secondary {
  background-color: transparent;
  color: var(--color-primary);
  border: 2px solid var(--color-primary);
}

/* Daily Wisdom Section */
.daily-wisdom {
  padding: 60px 0;
  background-color: #f0f5f5;
}

h2 {
  font-family: var(--font-heading);
  text-align: center;
  margin-bottom: 30px;
  color: var(--color-text);
}

.wisdom-card {
  background-color: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
  max-width: 800px;
  margin: 0 auto;
}

.wisdom-content {
  font-family: var(--font-heading);
  font-size: 1.3rem;
  line-height: 1.8;
  text-align: center;
  margin-bottom: 20px;
  font-style: italic;
  color: var(--color-text);
}

.wisdom-share {
  display: flex;
  justify-content: center;
}

.share-button {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  color: var(--color-primary);
  cursor: pointer;
  font-size: 1rem;
}

.share-icon {
  margin-right: 5px;
}

/* Features Section */
.features {
  padding: 60px 0;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
  margin-top: 40px;
}

.feature-card {
  background-color: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
  text-align: center;
  transition: transform 0.3s ease;
}

.feature-card:hover {
  transform: translateY(-5px);
}

.feature-icon {
  font-size: 2.5rem;
  margin-bottom: 20px;
}

.feature-card h3 {
  font-family: var(--font-heading);
  margin-bottom: 15px;
  color: var(--color-primary);
}

/* Testimonials Section */
.testimonials {
  padding: 60px 0;
  background-color: #f0f5f5;
}

.testimonial-carousel {
  max-width: 800px;
  margin: 0 auto;
}

.testimonial {
  background-color: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
  margin-bottom: 20px;
}

.testimonial-content {
  font-size: 1.1rem;
  line-height: 1.8;
  margin-bottom: 15px;
}

.testimonial-author {
  text-align: right;
  font-style: italic;
  color: #666;
}

/* About Page Styles */
.about-hero {
  padding: 60px 0;
}

.about-content {
  display: flex;
  flex-wrap: wrap;
  gap: 40px;
  margin-top: 30px;
}

.about-image {
  flex: 1;
  min-width: 300px;
  display: flex;
  justify-content: center;
}

.about-text {
  flex: 2;
  min-width: 300px;
}

.about-text p {
  margin-bottom: 20px;
}

/* Mission Section */
.mission {
  padding: 60px 0;
  background-color: #f0f5f5;
}

.mission-content {
  max-width: 900px;
  margin: 0 auto;
}

.mission-content > p {
  text-align: center;
  margin-bottom: 40px;
  font-size: 1.1rem;
}

.mission-points {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 30px;
}

.mission-point {
  background-color: white;
  border-radius: 10px;
  padding: 25px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
  text-align: center;
}

.point-icon {
  font-size: 2rem;
  margin-bottom: 15px;
}

.mission-point h3 {
  font-family: var(--font-heading);
  margin-bottom: 10px;
  color: var(--color-primary);
}

/* Join Movement Section */
.join-movement {
  padding: 60px 0;
}

.newsletter-signup {
  max-width: 500px;
  margin: 40px auto 0;
  background-color: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
  text-align: center;
}

.newsletter-signup h3 {
  margin-bottom: 20px;
  font-family: var(--font-heading);
}

.newsletter-form {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-bottom: 15px;
}

.newsletter-form input {
  flex: 1;
  min-width: 200px;
  padding: 12px 15px;
  border: 1px solid #ddd;
  border-radius: 30px;
  outline: none;
}

.form-note {
  font-size: 0.9rem;
  color: #666;
}

/* Contact Section */
.contact {
  padding: 60px 0;
  text-align: center;
  background-color: #f0f5f5;
}

.contact-link {
  display: inline-block;
  margin-top: 20px;
  color: var(--color-primary);
  font-size: 1.2rem;
  text-decoration: none;
  font-weight: 600;
}

/* Footer */
footer {
  background-color: #333;
  color: white;
  padding: 30px 0;
  text-align: center;
}

/* Animations */
.pulse {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(90, 125, 124, 0.7);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(90, 125, 124, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(90, 125, 124, 0);
  }
}

/* Responsive Styles */
@media (max-width: 768px) {
  .hero {
    flex-direction: column;
    text-align: center;
  }
  
  .hero-content {
    order: 1;
    padding: 0;
  }
  
  .cat-illustration {
    order: 2;
    margin-top: 30px;
  }
  
  .cta-buttons {
    justify-content: center;
  }
  
  h1 {
    font-size: 2.5rem;
  }
  
  #chat-container {
    width: 90%;
    right: 5%;
    left: 5%;
  }
}
```

#### JavaScript File (static/js/main.js)

Create a file at `/var/www/the-enlightened-cat/static/js/main.js`:

```javascript
document.addEventListener('DOMContentLoaded', function() {
    // Chat functionality
    const chatContainer = document.getElementById('chat-container');
    const openChatBtn = document.getElementById('open-chat');
    const closeChatBtn = document.getElementById('close-chat');
    const chatInput = document.getElementById('chat-input');
    const sendMessageBtn = document.getElementById('send-message');
    const chatMessages = document.getElementById('chat-messages');
    
    // Open chat
    openChatBtn.addEventListener('click', function() {
        chatContainer.classList.remove('hidden');
        openChatBtn.classList.add('hidden');
        chatInput.focus();
    });
    
    // Close chat
    closeChatBtn.addEventListener('click', function() {
        chatContainer.classList.add('hidden');
        openChatBtn.classList.remove('hidden');
    });
    
    // Send message function
    function sendMessage() {
        const message = chatInput.value.trim();
        if (message === '') return;
        
        // Add user message to chat
        addMessage('user', message);
        
        // Clear input
        chatInput.value = '';
        
        // Show typing indicator
        const typingIndicator = document.createElement('div');
        typingIndicator.className = 'message cat-message typing-indicator';
        typingIndicator.innerHTML = `
            <div class="avatar">🐈</div>
            <div class="message-content">
                <p>Thinking...</p>
            </div>
        `;
        chatMessages.appendChild(typingIndicator);
        
        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;
        
        // Send to API
        fetch('/api/chat', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ message: message }),
        })
        .then(response => response.json())
        .then(data => {
            // Remove typing indicator
            chatMessages.removeChild(typingIndicator);
            
            // Add cat response
            addMessage('cat', data.message);
        })
        .catch(error => {
            // Remove typing indicator
            chatMessages.removeChild(typingIndicator);
            
            // Add error message
            addMessage('cat', "I seem to be having a moment of contemplation. Could you try again?");
            console.error('Error:', error);
        });
    }
    
    // Add message to chat
    function addMessage(role, content) {
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${role}-message`;
        
        const avatar = document.createElement('div');
        avatar.className = 'avatar';
        avatar.textContent = role === 'user' ? '👤' : '🐈';
        
        const messageContent = document.createElement('div');
        messageContent.className = 'message-content';
        
        const paragraph = document.createElement('p');
        paragraph.textContent = content;
        
        messageContent.appendChild(paragraph);
        messageDiv.appendChild(avatar);
        messageDiv.appendChild(messageContent);
        
        chatMessages.appendChild(messageDiv);
        
        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }
    
    // Send message on button click
    sendMessageBtn.addEventListener('click', sendMessage);
    
    // Send message on Enter key
    chatInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            sendMessage();
        }
    });
    
    // Share buttons
    const shareButtons = document.querySelectorAll('.share-button');
    shareButtons.forEach(button => {
        button.addEventListener('click', function() {
            const platform = this.dataset.platform;
            const wisdom = document.querySelector('.wisdom-content p').textContent;
            let shareUrl = '';
            
            switch(platform) {
                case 'twitter':
                    shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(wisdom + ' - The Enlightened Cat')}&url=${encodeURIComponent(window.location.href)}`;
                    break;
                // Add more platforms as needed
            }
            
            if (shareUrl) {
                window.open(shareUrl, '_blank');
            }
        });
    });
    
    // Newsletter form
    const newsletterForm = document.getElementById('newsletter-form');
    if (newsletterForm) {
        newsletterForm.addEventListener('submit', function(e) {
            e.preventDefault();
            const email = document.getElementById('email').value;
            // Here you would normally send this to your backend
            alert('Thank you for subscribing! The Enlightened Cat will be in touch soon.');
            document.getElementById('email').value = '';
        });
    }
});
```

#### Create SVG Images

Create a basic SVG for the enlightened cat at `/var/www/the-enlightened-cat/static/images/enlightened-cat.svg`:

```bash
echo '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200">
  <style>
    .cat { fill: #5a7d7c; }
    .inner { fill: white; }
  </style>
  <circle cx="100" cy="100" r="90" fill="#f0f5f5" />
  <path class="cat" d="M100,20 C130,20 155,40 170,70 C175,80 180,95 180,110 C180,155 145,180 100,180 C55,180 20,155 20,110 C20,95 25,80 30,70 C45,40 70,20 100,20 Z" />
  <circle class="inner" cx="70" cy="90" r="10" />
  <circle class="inner" cx="130" cy="90" r="10" />
  <path class="inner" d="M65,130 Q100,150 135,130" stroke="white" stroke-width="3" fill="none" />
  <path d="M50,60 L30,40 M150,60 L170,40" stroke="#5a7d7c" stroke-width="3" />
  <path d="M80,110 L100,120 L120,110" stroke="#f0f5f5" stroke-width="2" fill="none" />
</svg>' > /var/www/the-enlightened-cat/static/images/enlightened-cat.svg
```

Create a story SVG at `/var/www/the-enlightened-cat/static/images/cat-story.svg`:

```bash
echo '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200">
  <style>
    .cat { fill: #5a7d7c; }
    .bg { fill: #f0f5f5; }
    .item { fill: #dadada; }
  </style>
  <rect width="200" height="200" fill="white" />
  <rect class="bg" x="20" y="20" width="160" height="160" rx="10" />
  <rect class="item" x="40" y="40" width="30" height="30" rx="5" />
  <rect class="item" x="80" y="40" width="80" height="10" rx="5" />
  <rect class="item" x="80" y="60" width="60" height="10" rx="5" />
  <path class="cat" d="M100,100 C120,100 135,110 140,125 C142,130 145,140 145,145 C145,165 125,180 100,180 C75,180 55,165 55,145 C55,140 58,130 60,125 C65,110 80,100 100,100 Z" />
  <circle fill="white" cx="85" cy="130" r="5" />
  <circle fill="white" cx="115" cy="130" r="5" />
  <path d="M90,145 Q100,155 110,145" stroke="white" stroke-width="2" fill="none" />
  <path d="M75,115 L65,105 M125,115 L135,105" stroke="#5a7d7c" stroke-width="2" />
</svg>' > /var/www/the-enlightened-cat/static/images/cat-story.svg
```

### Create Environment File

```bash
cat > /var/www/the-enlightened-cat/.env << EOF
MISTRAL_API_KEY=your_mistral_api_key
MISTRAL_API_URL=https://api.mistral.ai/v1
PORT=9000
RUST_LOG=info
EOF
```

### Build the Application

```bash
cd /var/www/the-enlightened-cat
cargo build --release
```

## 3. Set Up Systemd Service

```bash
sudo tee /etc/systemd/system/enlightened-cat.service > /dev/null << EOF
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
EOF

sudo systemctl daemon-reload
sudo systemctl enable enlightened-cat
sudo systemctl start enlightened-cat
```

## 4. Configure Nginx

```bash
sudo tee /etc/nginx/sites-available/the-enlightened-cat > /dev/null << EOF
server {
    listen 80;
    server_name the-enlightened-cat.com www.the-enlightened-cat.com;

    location / {
        proxy_pass http://127.0.0.1:9000;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

    location /static {
        alias /var/www/the-enlightened-cat/static;
        expires 1d;
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/the-enlightened-cat /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

## 5. Set Up SSL with Let's Encrypt

```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d the-enlightened-cat.com -d www.the-enlightened-cat.com
```

## 6. Monitoring and Maintenance

### Check Service Status

```bash
sudo systemctl status enlightened-cat
```

### View Logs

```bash
sudo journalctl -u enlightened-cat -f
```

### Restart Service After Updates

```bash
cd /var/www/the-enlightened-cat
git pull
cargo build --release
sudo systemctl restart enlightened-cat
```

## 7. Backup Strategy

### Backup Configuration Files

```bash
sudo cp /etc/nginx/sites-available/the-enlightened-cat /backup/
sudo cp /etc/systemd/system/enlightened-cat.service /backup/
```

### Backup Application Code

```bash
cd /var/www
sudo tar -czf /backup/enlightened-cat-$(date +%Y%m%d).tar.gz the-enlightened-cat
```

## Troubleshooting

### Application Won't Start

Check logs:
```bash
sudo journalctl -u enlightened-cat -n 50
```

Common issues:
- Incorrect API key
- Port already in use
- Missing environment variables

### Nginx Configuration Issues

Test configuration:
```bash
sudo nginx -t
```

Check Nginx logs:
```bash
sudo tail -f /var/log/nginx/error.log
```

### SSL Certificate Issues

Renew certificates:
```bash
sudo certbot renew
```
