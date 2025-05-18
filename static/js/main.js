document.addEventListener('DOMContentLoaded', function() {
    // Mobile menu toggle
    const mobileMenuToggle = document.getElementById('mobile-menu-toggle');
    const mainNav = document.getElementById('main-nav');
    
    if (mobileMenuToggle && mainNav) {
        mobileMenuToggle.addEventListener('click', function() {
            mainNav.classList.toggle('active');
            mobileMenuToggle.classList.toggle('active');
        });
        
        // Close menu when clicking on a link
        const navLinks = mainNav.querySelectorAll('a');
        navLinks.forEach(link => {
            link.addEventListener('click', function() {
                mainNav.classList.remove('active');
                mobileMenuToggle.classList.remove('active');
            });
        });
    }
    
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
    
    // Track conversation state
    let conversationDepth = 0;
    let currentTopic = null;
    let suggestedTopics = [];
    
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
        
        // Send to API with conversation metadata
        fetch('/api/chat', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                message: message,
                conversationDepth: conversationDepth,
                currentTopic: currentTopic
            }),
        })
        .then(response => response.json())
        .then(data => {
            // Remove typing indicator
            chatMessages.removeChild(typingIndicator);
            
            // Add cat response
            addMessage('cat', data.message);
            
            // Update conversation depth
            conversationDepth++;
            
            // After a few exchanges, offer topic exploration
            if (conversationDepth === 3) {
                setTimeout(() => {
                    addTopicExplorationPrompt();
                }, 1000);
            }
            
            // If the response includes suggested topics, store them
            if (data.suggestedTopics && data.suggestedTopics.length > 0) {
                suggestedTopics = data.suggestedTopics;
                if (suggestedTopics.length > 0) {
                    setTimeout(() => {
                        addTopicSuggestions(suggestedTopics);
                    }, 1000);
                }
            }
        })
        .catch(error => {
            // Remove typing indicator
            chatMessages.removeChild(typingIndicator);
            
            // Add error message
            addMessage('cat', "I seem to be having a moment of contemplation. Could you try again?");
            console.error('Error:', error);
        });
    }
    
    // Add topic exploration prompt
    function addTopicExplorationPrompt() {
        const messageDiv = document.createElement('div');
        messageDiv.className = 'message cat-message topic-prompt';
        
        const avatar = document.createElement('div');
        avatar.className = 'avatar';
        avatar.textContent = '🐈';
        
        const messageContent = document.createElement('div');
        messageContent.className = 'message-content';
        
        const paragraph = document.createElement('p');
        paragraph.textContent = "I sense you might want to explore this topic more deeply. Would you like to have a more in-depth conversation about what's on your mind?";
        
        const buttonContainer = document.createElement('div');
        buttonContainer.className = 'topic-buttons';
        
        const yesButton = document.createElement('button');
        yesButton.textContent = 'Yes, let\'s go deeper';
        yesButton.addEventListener('click', () => {
            // Remove the prompt
            chatMessages.removeChild(messageDiv);
            // Send a message to transition to deeper conversation
            chatInput.value = "Yes, I'd like to explore this more deeply.";
            sendMessage();
        });
        
        const noButton = document.createElement('button');
        noButton.textContent = 'No, just zen wisdom';
        noButton.addEventListener('click', () => {
            // Remove the prompt
            chatMessages.removeChild(messageDiv);
            // Send a message to continue with regular wisdom
            chatInput.value = "I prefer to keep our conversation light and zen.";
            sendMessage();
        });
        
        buttonContainer.appendChild(yesButton);
        buttonContainer.appendChild(noButton);
        
        messageContent.appendChild(paragraph);
        messageContent.appendChild(buttonContainer);
        messageDiv.appendChild(avatar);
        messageDiv.appendChild(messageContent);
        
        chatMessages.appendChild(messageDiv);
        
        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }
    
    // Add topic suggestions
    function addTopicSuggestions(topics) {
        const messageDiv = document.createElement('div');
        messageDiv.className = 'message cat-message topic-suggestions';
        
        const avatar = document.createElement('div');
        avatar.className = 'avatar';
        avatar.textContent = '🐈';
        
        const messageContent = document.createElement('div');
        messageContent.className = 'message-content';
        
        const paragraph = document.createElement('p');
        paragraph.textContent = "Would you like to explore any of these topics?";
        
        const topicContainer = document.createElement('div');
        topicContainer.className = 'topic-buttons';
        
        topics.forEach(topic => {
            const topicButton = document.createElement('button');
            topicButton.textContent = topic;
            topicButton.addEventListener('click', () => {
                // Set current topic
                currentTopic = topic;
                // Send a message to explore this topic
                chatInput.value = `Let's talk about ${topic}`;
                sendMessage();
                // Remove the suggestions
                chatMessages.removeChild(messageDiv);
            });
            topicContainer.appendChild(topicButton);
        });
        
        messageContent.appendChild(paragraph);
        messageContent.appendChild(topicContainer);
        messageDiv.appendChild(avatar);
        messageDiv.appendChild(messageContent);
        
        chatMessages.appendChild(messageDiv);
        
        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;
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
    
    // Share buttons for Twitter and Facebook only
    // LinkedIn is handled separately in linkedin-share.js
    const shareButtons = document.querySelectorAll('.twitter-share, .facebook-share');
    shareButtons.forEach(button => {
        button.addEventListener('click', function() {
            const platform = this.dataset.platform;
            const wisdom = document.querySelector('.wisdom-content p').textContent;
            let shareUrl = '';
            
            switch(platform) {
                case 'twitter':
                    shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(wisdom + ' - The Enlightened Cat')}&url=${encodeURIComponent(window.location.href)}`;
                    break;
                case 'facebook':
                    shareUrl = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(window.location.href)}&quote=${encodeURIComponent(wisdom)}`;
                    break;
            }
            
            if (shareUrl) {
                window.open(shareUrl, '_blank');
            }
        });
    });
    
    // Note: LinkedIn sharing is handled in linkedin-share.js
    
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
