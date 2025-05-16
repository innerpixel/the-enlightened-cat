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
                case 'facebook':
                    shareUrl = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(window.location.href)}&quote=${encodeURIComponent(wisdom)}`;
                    break;
                case 'linkedin':
                    shareUrl = `https://www.linkedin.com/shareArticle?mini=true&url=${encodeURIComponent(window.location.href)}&title=${encodeURIComponent('Wisdom from The Enlightened Cat')}&summary=${encodeURIComponent(wisdom)}`;
                    break;
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
