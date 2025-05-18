/**
 * LinkedIn Share Modal Functionality
 * Enhances the LinkedIn sharing experience by providing a preview and customization options
 * while maintaining compatibility with the existing sharing system
 */
document.addEventListener('DOMContentLoaded', function() {
    // Create the LinkedIn share modal HTML structure
    function createLinkedInModal() {
        const modal = document.createElement('div');
        modal.className = 'linkedin-share-modal';
        modal.id = 'linkedin-share-modal';
        
        modal.innerHTML = `
            <div class="linkedin-modal-content">
                <div class="preview-header">
                    <div>
                        <svg class="linkedin-logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white">
                            <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                        </svg>
                        Share to LinkedIn
                    </div>
                    <button class="close-button" id="linkedin-close-button">&times;</button>
                </div>
                <div class="preview-content">
                    <div class="preview-image">
                        <img src="/static/images/enlightened-cat.svg" alt="The Enlightened Cat">
                    </div>
                    <div class="preview-title">Wisdom from The Enlightened Cat</div>
                    <div class="preview-subtitle">Finding peace in the professional jungle</div>
                    <div class="preview-text" id="linkedin-preview-text"></div>
                    <div class="preview-source"><a href="https://the-enlightened-cat.com" target="_blank">https://the-enlightened-cat.com</a></div>
                </div>
                <div class="comment-section">
                    <label for="linkedin-comment">Add your thoughts (optional):</label>
                    <textarea id="linkedin-comment" placeholder="What resonated with you about this wisdom?"></textarea>
                </div>
                <div class="preview-actions">
                    <button class="cancel" id="linkedin-cancel-button">Cancel</button>
                    <button class="share" id="linkedin-share-button">Share on LinkedIn</button>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        // Add event listeners
        document.getElementById('linkedin-close-button').addEventListener('click', closeLinkedInModal);
        document.getElementById('linkedin-cancel-button').addEventListener('click', closeLinkedInModal);
        document.getElementById('linkedin-share-button').addEventListener('click', shareToLinkedIn);
        
        // Close modal when clicking outside content
        modal.addEventListener('click', function(e) {
            if (e.target === modal) {
                closeLinkedInModal();
            }
        });
    }
    
    // Create the modal when the page loads
    createLinkedInModal();
    
    // Function to open the LinkedIn share modal
    function openLinkedInModal(wisdom) {
        const modal = document.getElementById('linkedin-share-modal');
        const previewText = document.getElementById('linkedin-preview-text');
        
        // Set the wisdom text in the preview
        previewText.textContent = wisdom;
        
        // Clear any previous comment
        document.getElementById('linkedin-comment').value = '';
        
        // Show the modal with animation
        modal.classList.add('active');
        
        // Prevent body scrolling
        document.body.style.overflow = 'hidden';
    }
    
    // Function to close the LinkedIn share modal
    function closeLinkedInModal() {
        const modal = document.getElementById('linkedin-share-modal');
        modal.classList.remove('active');
        
        // Restore body scrolling
        document.body.style.overflow = '';
    }
    
    // Function to share to LinkedIn
    function shareToLinkedIn() {
        const wisdom = document.getElementById('linkedin-preview-text').textContent;
        const comment = document.getElementById('linkedin-comment').value;
        const shareButton = document.getElementById('linkedin-share-button');
        
        // Show loading state
        shareButton.classList.add('loading');
        shareButton.textContent = 'Sharing...';
        
        // Build the share text (combining comment and wisdom)
        const shareText = comment ? 
            `${comment}\n\n"${wisdom}" - The Enlightened Cat` : 
            `"${wisdom}" - The Enlightened Cat`;
        
        // Store the comment for potential future API integration
        if (comment) {
            console.log('User comment:', comment);
            // In a future version with LinkedIn API, we could use this comment directly
        }
        
        // Simulate a slight delay for better UX
        setTimeout(() => {
            // Use the newer LinkedIn sharing API endpoint
            // LinkedIn now primarily relies on OpenGraph tags for content info
            const linkedInShareUrl = `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(window.location.href)}`;
            
            // Open in a popup window with specific dimensions for better UX
            window.open(linkedInShareUrl, 'LinkedInShare', 'width=800,height=600');
            
            // Reset button state
            shareButton.classList.remove('loading');
            shareButton.textContent = 'Share on LinkedIn';
            
            // Close the modal
            closeLinkedInModal();
            
            // Show a success message
            showShareConfirmation();
        }, 800);
    }
    
    // Show a confirmation message after sharing
    function showShareConfirmation() {
        // Create a toast notification
        const toast = document.createElement('div');
        toast.className = 'share-toast';
        toast.textContent = 'Sharing to LinkedIn...';
        document.body.appendChild(toast);
        
        // Animate in
        setTimeout(() => {
            toast.classList.add('visible');
        }, 10);
        
        // Remove after delay
        setTimeout(() => {
            toast.classList.remove('visible');
            setTimeout(() => {
                document.body.removeChild(toast);
            }, 300);
        }, 3000);
    }
    
    // Add LinkedIn share button behavior
    const linkedInButtons = document.querySelectorAll('.linkedin-share');
    linkedInButtons.forEach(button => {
        // Add our custom click handler
        button.addEventListener('click', function(e) {
            e.preventDefault(); // Prevent default link behavior
            
            // Get the wisdom text
            let wisdom = '';
            const wisdomElement = document.querySelector('.wisdom-content p');
            if (wisdomElement) {
                wisdom = wisdomElement.textContent;
                openLinkedInModal(wisdom);
            } else {
                // Open LinkedIn sharing dialog using the newer API endpoint
                // LinkedIn now primarily relies on OpenGraph tags for content info
                const linkedInShareUrl = `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(window.location.href)}`;
                
                // Open in a popup window with specific dimensions for better UX
                window.open(linkedInShareUrl, 'LinkedInShare', 'width=800,height=600');
            }
        });
    });
});
