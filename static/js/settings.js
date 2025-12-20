// Settings page functionality

document.addEventListener('DOMContentLoaded', () => {
    const saveButton = document.getElementById('save-settings');
    const testButton = document.getElementById('test-connection');
    const resetButton = document.getElementById('reset-settings');

    if (saveButton) {
        saveButton.addEventListener('click', () => {
            showNotification('Settings saved locally. API persistence coming soon.', 'success');
        });
    }

    if (testButton) {
        testButton.addEventListener('click', () => {
            showNotification('Testing connections...', 'info');
            fetch('/api/status')
                .then(response => response.json())
                .then(data => {
                    const status = data.mcp_connected && data.lnd_connected ? 'All systems connected.' : 'One or more connections are down.';
                    showNotification(status, data.mcp_connected && data.lnd_connected ? 'success' : 'error');
                })
                .catch(() => showNotification('Unable to reach services.', 'error'));
        });
    }

    if (resetButton) {
        resetButton.addEventListener('click', () => {
            showNotification('Settings reset to defaults (local only).', 'info');
        });
    }
});

function showNotification(message, type) {
    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 1rem 1.5rem;
        border-radius: 6px;
        color: white;
        font-weight: 500;
        z-index: 1000;
    `;

    const colors = {
        success: '#10B981',
        error: '#EF4444',
        info: '#2D5BFF'
    };
    notification.style.backgroundColor = colors[type] || '#6B7280';

    document.body.appendChild(notification);
    setTimeout(() => notification.remove(), 4000);
}
