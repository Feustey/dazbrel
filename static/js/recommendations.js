// Recommendations page functionality

document.addEventListener('DOMContentLoaded', () => {
    setupFilters();
    setupActions();
});

function setupFilters() {
    const priorityFilter = document.getElementById('priority-filter');
    const typeFilter = document.getElementById('type-filter');

    const applyFilters = () => {
        const priorityValue = priorityFilter.value;
        const typeValue = typeFilter.value;
        const cards = document.querySelectorAll('.recommendation-card');

        cards.forEach(card => {
            const matchesPriority = priorityValue === 'all' || card.dataset.priority === priorityValue;
            const matchesType = typeValue === 'all' || card.dataset.type === typeValue;
            card.style.display = matchesPriority && matchesType ? 'block' : 'none';
        });
    };

    priorityFilter.addEventListener('change', applyFilters);
    typeFilter.addEventListener('change', applyFilters);
}

function setupActions() {
    document.addEventListener('click', (event) => {
        if (event.target.classList.contains('btn-approve')) {
            handleAction(event.target.dataset.id, 'Approve');
        }
        if (event.target.classList.contains('btn-reject')) {
            handleAction(event.target.dataset.id, 'Reject');
        }
        if (event.target.classList.contains('btn-details')) {
            showNotification('Detailed view is coming soon.', 'info');
        }
    });
}

function handleAction(recommendationId, action) {
    fetch('/api/actions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ recommendation_id: recommendationId, action })
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            showNotification(`Recommendation ${action.toLowerCase()}ed`, 'success');
        })
        .catch(() => {
            showNotification('Unable to execute action right now.', 'error');
        });
}

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
