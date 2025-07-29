// Dashboard JavaScript functionality
document.addEventListener('DOMContentLoaded', function() {
    // Initialize dashboard
    initDashboard();
    
    // Set up real-time updates
    setInterval(updateMetrics, 30000); // Update every 30 seconds
    
    // Handle recommendation actions
    setupRecommendationHandlers();
});

function initDashboard() {
    console.log('Dazno Dashboard initialized');
    
    // Check connection status
    checkConnectionStatus();
    
    // Load initial data
    loadDashboardData();
}

function updateMetrics() {
    fetch('/api/metrics')
        .then(response => response.json())
        .then(data => {
            updateMetricCards(data);
        })
        .catch(error => {
            console.error('Error updating metrics:', error);
        });
}

function updateMetricCards(data) {
    // Update metric values in the UI
    const metricElements = document.querySelectorAll('.metric-value');
    metricElements.forEach(element => {
        const metricType = element.closest('.metric-card').querySelector('.metric-label').textContent;
        if (data[metricType]) {
            element.textContent = data[metricType];
        }
    });
}

function setupRecommendationHandlers() {
    // Handle approve buttons
    document.querySelectorAll('.btn-approve').forEach(button => {
        button.addEventListener('click', function() {
            const recommendationId = this.dataset.id;
            approveRecommendation(recommendationId);
        });
    });
    
    // Handle reject buttons
    document.querySelectorAll('.btn-reject').forEach(button => {
        button.addEventListener('click', function() {
            const recommendationId = this.dataset.id;
            rejectRecommendation(recommendationId);
        });
    });
}

function approveRecommendation(id) {
    if (confirm('Are you sure you want to approve and execute this recommendation?')) {
        fetch('/api/actions', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                recommendation_id: id,
                action: 'Approve'
            })
        })
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                showNotification('Recommendation approved successfully', 'success');
                removeRecommendationCard(id);
            } else {
                showNotification('Failed to approve recommendation: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showNotification('Network error occurred', 'error');
        });
    }
}

function rejectRecommendation(id) {
    if (confirm('Are you sure you want to reject this recommendation?')) {
        fetch('/api/actions', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                recommendation_id: id,
                action: 'Reject'
            })
        })
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                showNotification('Recommendation rejected', 'info');
                removeRecommendationCard(id);
            } else {
                showNotification('Failed to reject recommendation: ' + data.message, 'error');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            showNotification('Network error occurred', 'error');
        });
    }
}

function removeRecommendationCard(id) {
    const card = document.querySelector(`[data-id="${id}"]`).closest('.recommendation-card');
    if (card) {
        card.style.transform = 'translateX(-100%)';
        card.style.opacity = '0';
        setTimeout(() => card.remove(), 300);
    }
}

function checkConnectionStatus() {
    fetch('/api/status')
        .then(response => response.json())
        .then(data => {
            updateConnectionStatus(data.mcp_connected, data.lnd_connected);
        })
        .catch(error => {
            console.error('Error checking status:', error);
            updateConnectionStatus(false, false);
        });
}

function updateConnectionStatus(mcpConnected, lndConnected) {
    const statusIndicator = document.querySelector('.status-indicator');
    if (mcpConnected && lndConnected) {
        statusIndicator.classList.add('connected');
    } else {
        statusIndicator.classList.remove('connected');
    }
}

function loadDashboardData() {
    fetch('/api/dashboard')
        .then(response => response.json())
        .then(data => {
            console.log('Dashboard data loaded:', data);
        })
        .catch(error => {
            console.error('Error loading dashboard data:', error);
        });
}

function showNotification(message, type) {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    
    // Style the notification
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 1rem 1.5rem;
        border-radius: 6px;
        color: white;
        font-weight: 500;
        z-index: 1000;
        animation: slideIn 0.3s ease;
    `;
    
    // Set background color based on type
    switch(type) {
        case 'success':
            notification.style.backgroundColor = '#10B981';
            break;
        case 'error':
            notification.style.backgroundColor = '#EF4444';
            break;
        case 'info':
            notification.style.backgroundColor = '#2D5BFF';
            break;
        default:
            notification.style.backgroundColor = '#6B7280';
    }
    
    // Add to page
    document.body.appendChild(notification);
    
    // Remove after 5 seconds
    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease';
        setTimeout(() => notification.remove(), 300);
    }, 5000);
}

// Add CSS animations
const style = document.createElement('style');
style.textContent = `
    @keyframes slideIn {
        from { transform: translateX(100%); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
    }
    
    @keyframes slideOut {
        from { transform: translateX(0); opacity: 1; }
        to { transform: translateX(100%); opacity: 0; }
    }
`;
document.head.appendChild(style);